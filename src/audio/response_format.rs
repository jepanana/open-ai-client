use std::fmt::{self, Display, Formatter};

/// The format of the audio output, in one of these options: json, text, srt, verbose_json, or vt
#[derive(Clone, Debug, Default, PartialEq)]
pub enum ResponseFormat {
    #[default]
    /// JSON format output
    Json,

    /// Plain text output
    Text,

    /// SubRip subtitle format
    SubRipSubtitle,

    /// Verbose JSON format output
    VerboseJson,

    /// Variable Valve Timing format output
    VariableValveTiming,
}

impl Display for ResponseFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseFormat::Json => write!(f, "json"),
            ResponseFormat::Text => write!(f, "text"),
            ResponseFormat::SubRipSubtitle => write!(f, "srt"),
            ResponseFormat::VerboseJson => write!(f, "verbose_json"),
            ResponseFormat::VariableValveTiming => write!(f, "vvt"),
        }
    }
}
