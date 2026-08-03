use atomic_enum::atomic_enum;
use std::sync::atomic::{AtomicBool, Ordering};

static RENDER_MODE: AtomicRenderMode = AtomicRenderMode::new(RenderMode::Normal);
static IS_QUIET: AtomicBool = AtomicBool::new(false);

#[atomic_enum]
#[derive(PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Json,
    Plain,
}

pub fn set_is_quiet(quiet: bool) {
    IS_QUIET.store(quiet, Ordering::Relaxed);
}

#[inline]
pub fn is_quiet() -> bool {
    IS_QUIET.load(Ordering::Relaxed)
}

pub fn set_render_mode(render_mode: RenderMode) {
    RENDER_MODE.store(render_mode, Ordering::Relaxed);
}

#[inline]
pub fn get_render_mode() -> RenderMode {
    return RENDER_MODE.load(Ordering::Relaxed);
}

#[allow(clippy::disallowed_macros)]
mod println_macros {
    #[macro_export]
    macro_rules! cli_println {
        ($($arg:tt)*) => {{
            if $crate::util::io::get_render_mode() != $crate::util::io::RenderMode::Json && !$crate::util::io::is_quiet() {
                let colored_message = std::format!($($arg)*);
                let stripped_message = strip_ansi_escapes::strip_str(colored_message.clone());
                let message = owo_colors::OwoColorize::if_supports_color(
                    &stripped_message,
                    owo_colors::Stream::Stdout,
                    |_| colored_message.clone(),
                );
                std::println!("{}", message);
            }
        }};
    }
    #[macro_export]
    macro_rules! cli_eprintln {
        (force: true, $($arg:tt)*) => {{
            let colored_message = std::format!($($arg)*);
                let stripped_message = strip_ansi_escapes::strip_str(colored_message.clone());
                let message = owo_colors::OwoColorize::if_supports_color(
                    &stripped_message,
                    owo_colors::Stream::Stderr,
                    |_| colored_message.clone(),
                );
                std::println!("{}", message);
        }};
        ($($arg:tt)*) => {{
            if $crate::util::io::get_render_mode() != $crate::util::io::RenderMode::Json && !$crate::util::io::is_quiet() {
                $crate::cli_eprintln!(force: true, $($arg)*);
            }
        }}
    }
}
