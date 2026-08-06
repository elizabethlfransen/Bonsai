const BONSAI_FLAG_PREFIX: &str = "BONSAI";

#[inline]
fn is_bonsai_flag_set(flag_name: &str) -> bool {
    std::env::var(format_bonsai_flag_name(flag_name)).is_ok()
}

#[inline]
fn format_bonsai_flag_name(flag_name: &str) -> String {
    format!("{BONSAI_FLAG_PREFIX}_{}", flag_name.to_uppercase())
}

macro_rules! env_set_flags {
    ($($item:ident),*$(,)?) => {
        $(
            #[inline]
            pub fn $item() -> bool {
                is_bonsai_flag_set(stringify!($item))
            }
        )*
    };
}

env_set_flags!(no_color, no_input, force_color,);
