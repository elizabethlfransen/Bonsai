use clap::Command;
use miette::Diagnostic;
use thiserror::Error;


pub struct Example {
    pub description: String,
    pub usage: String,
}

#[derive(Debug, Error, Diagnostic)]
pub enum ExampleParseError {
    #[error("Example is missing description")]
    #[diagnostic(code(bonsai::example::parse::description_missing))]
    DescriptionMissing,

    #[error("Example is missing usage")]
    #[diagnostic(code(bonsai::example::parse::usage_missing))]
    UsageMissing,
}

impl Example {
    pub fn extract_from_examples_section(
        examples_section: &str,
    ) -> Result<Vec<Self>, ExampleParseError> {
        const EXAMPLE_HEADER: &str = "Examples:\n";

        Ok(
            if let Some(stripped_example_section) = examples_section.strip_prefix(EXAMPLE_HEADER) {
                stripped_example_section
                    .split("\n\n")
                    .filter(|example| !example.trim().is_empty())
                    .map(Example::parse_example)
                    .collect::<Result<_, _>>()?
            } else {
                Vec::new()
            },
        )
    }

    fn parse_example(example_section: &str) -> Result<Self, ExampleParseError> {
        let mut description = Option::None;
        let mut usage = Option::None;
        for line in example_section.lines() {
            if let Some(usage_text) = line.strip_prefix("  $ ") {
                usage = Some(usage_text.to_string());
            }
            if let Some(description_text) = line.strip_prefix("  # ") {
                description = Some(description_text.to_string());
            }
        }
        Ok(Self {
            description: description.ok_or(ExampleParseError::DescriptionMissing)?,
            usage: usage.ok_or(ExampleParseError::UsageMissing)?,
        })
    }
}

pub trait ToExamples {
    fn get_examples(&self) -> Result<Vec<Example>, ExampleParseError>;
}

impl ToExamples for Command {
    fn get_examples(&self) -> Result<Vec<Example>, ExampleParseError> {
        let after_help = self
            .get_after_long_help()
            .or(self.get_after_help())
            .map(|x| x.to_string())
            .unwrap_or_default();

        Example::extract_from_examples_section(&after_help)
    }
}
