use atomic_enum::atomic_enum;
use miette::{MietteHandler, MietteHandlerOpts, NarratableReportHandler};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::util::report_handler::{
    MinimalJsonReportHandler, MinimalReportHandler, WrappedJsonReportHandler,
};

static RENDER_MODE: AtomicRenderMode = AtomicRenderMode::new(RenderMode::Normal);
static IS_QUIET: AtomicBool = AtomicBool::new(false);
static NO_COLOR: AtomicBool = AtomicBool::new(false);
static FORCE_COLOR: AtomicBool = AtomicBool::new(false);

#[atomic_enum]
#[derive(PartialEq, Eq)]
pub enum RenderMode {
    Normal,
    Json,
    Plain,
}

pub fn set_color_override(enabled: bool) {
    NO_COLOR.store(!enabled, Ordering::Relaxed);
    FORCE_COLOR.store(enabled, Ordering::Relaxed);
    owo_colors::set_override(enabled);
    console::set_colors_enabled(enabled);
    console::set_colors_enabled_stderr(enabled);
}

pub fn set_is_quiet(quiet: bool) {
    IS_QUIET.store(quiet, Ordering::Relaxed);
}

pub fn setup_miettte() -> miette::Result<()> {
    miette::set_hook(Box::new(move |_| {
        let render_mode = RENDER_MODE.load(Ordering::Relaxed);
        match (is_quiet(), render_mode) {
            (true, RenderMode::Json) => Box::new(MinimalJsonReportHandler),
            (true, _) => Box::new(MinimalReportHandler),
            (_, RenderMode::Normal) => Box::new(build_standard_reporter()),
            (_, RenderMode::Json) => Box::new(WrappedJsonReportHandler::new()),
            (_, RenderMode::Plain) => Box::new(NarratableReportHandler::new()),
        }
    }))?;
    Ok(())
}

fn build_standard_reporter() -> MietteHandler {
    let mut opts = MietteHandlerOpts::new();
    let force_color = FORCE_COLOR.load(Ordering::Relaxed);
    let no_color = NO_COLOR.load(Ordering::Relaxed);
    if no_color {
        opts = opts.color(false);
    }
    if force_color {
        opts = opts.color(true);
    }
    opts.build()
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
    RENDER_MODE.load(Ordering::Relaxed)
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
