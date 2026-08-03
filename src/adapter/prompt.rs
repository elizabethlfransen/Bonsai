pub use auto_confirm_wrapper::*;
pub use cliclack_adapater::*;
use miette::Diagnostic;
pub use non_interactive_adapter::*;
use thiserror::Error;

mod cliclack_adapater {
    use super::{Confirm, PromptAdapter, Result};

    /// A prompt adapter which is backed by CliClack.
    pub struct CliClackPromptAdapter {}
    pub struct CliClackConfirm(cliclack::Confirm);

    impl Confirm for CliClackConfirm {
        fn interact(&mut self) -> Result<bool> {
            Ok(self.0.interact()?)
        }
    }

    impl PromptAdapter for CliClackPromptAdapter {
        fn confirm(&self, prompt: &str) -> Box<dyn Confirm> {
            Box::new(CliClackConfirm(cliclack::confirm(prompt)))
        }
    }
}
mod non_interactive_adapter {

    use super::{Confirm, PromptAdapter, PromptError, Result};
    /// A prompt adapter which returns an error any time the user should be prompted
    struct NonInteractivePrompt;

    impl Confirm for NonInteractivePrompt {
        fn interact(&mut self) -> Result<bool> {
            Err(PromptError::NonInteractive)
        }
    }

    pub struct NonInteractivePromptAdapter;
    impl PromptAdapter for NonInteractivePromptAdapter {
        fn confirm(&self, _: &str) -> Box<dyn Confirm> {
            Box::new(NonInteractivePrompt)
        }
    }
}
mod auto_confirm_wrapper {
    use super::{Confirm, PromptAdapter, Result};
    pub struct AutoConfirmPromptAdapter<T: PromptAdapter> {
        #[allow(dead_code)]
        internal: T,
        auto_value: bool,
    }
    struct AutoConfirm(bool);

    impl Confirm for AutoConfirm {
        fn interact(&mut self) -> Result<bool> {
            return Ok(self.0);
        }
    }
    impl<T: PromptAdapter> AutoConfirmPromptAdapter<T> {
        pub fn new(prompt_adapter: T, auto_value: bool) -> Self {
            return Self {
                internal: prompt_adapter,
                auto_value,
            };
        }
    }
    impl<T: PromptAdapter> PromptAdapter for AutoConfirmPromptAdapter<T> {
        fn confirm(&self, _: &str) -> Box<dyn Confirm> {
            Box::new(AutoConfirm(self.auto_value))
        }
    }
}

/// An adapter for handling prompting to the user.
pub trait PromptAdapter {
    fn confirm(&self, prompt: &str) -> Box<dyn Confirm>;
}
pub trait Confirm {
    fn interact(&mut self) -> Result<bool>;
}

pub type Result<T> = std::result::Result<T, PromptError>;

#[derive(Error, Debug, Diagnostic)]
pub enum PromptError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Attempted to prompt in a non-interactive terminal")]
    #[diagnostic(code(bonsai::prompt::non_interactive))]
    NonInteractive,
}
