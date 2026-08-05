#[macro_export]
macro_rules! examples {
    ($($description:expr => $usage:expr),*$(,)?) => {
        concat!(
            "\x1b[1;4mExamples:\x1b[0m\n",
            $(
                "  # ", $description, "\n",
                "  $ ", $usage, "\n\n"
            ),*
        )
    };
}
