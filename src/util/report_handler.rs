use miette::{JSONReportHandler, ReportHandler};
use owo_colors::{OwoColorize, Stream};
use serde_json::{Value, json};

const UNKNOWN_ERROR: &str = "bonsai::error::unknown";

/// A minimal error reporter which only reports the error code
pub struct MinimalReportHandler;

impl ReportHandler for MinimalReportHandler {
    fn debug(
        &self,
        error: &dyn miette::Diagnostic,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write!(
            f,
            "{}",
            error
                .code()
                .unwrap_or(Box::new(UNKNOWN_ERROR))
                .if_supports_color(Stream::Stderr, |text| text.red())
        )
    }
}

pub struct MinimalJsonReportHandler;
impl ReportHandler for MinimalJsonReportHandler {
    fn debug(
        &self,
        error: &dyn miette::Diagnostic,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let code = error.code().unwrap_or(Box::new(UNKNOWN_ERROR)).to_string();
        let output = json!({
            "result": "error",
            "error": {
                "code": code
            }
        });
        write!(f, "{output}")
    }
}

pub struct WrappedJsonReportHandler(JSONReportHandler);

impl WrappedJsonReportHandler {
    pub fn new() -> Self {
        WrappedJsonReportHandler(JSONReportHandler)
    }
}

impl ReportHandler for WrappedJsonReportHandler {
    fn debug(
        &self,
        error: &dyn miette::Diagnostic,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let mut raw_json = String::new();
        self.0.render_report(&mut raw_json, error)?;
        let parsed_error: Value = serde_json::from_str(&raw_json).unwrap();
        let wrapped_error = json!({
            "result": "error",
            "error": parsed_error
        });
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&wrapped_error).unwrap()
        )
    }
}
