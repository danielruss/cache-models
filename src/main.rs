use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use soccer_rs::{MODEL_CONFIG, ModelType, SoccerBuilder, SoccerPipeline};

#[derive(Parser, Debug)]
#[command(
    version,
    name = "cache-models",
    subcommand_value_name = "CLASSIFIER",
    subcommand_help_heading = "Classifiers",
    about = "Cache SOCcerNET/CLIPS models"
)]
struct Cli {
    #[command(subcommand)]
    classifier: Classifier,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum SoccernetVersion {
    #[value(name = "1.0.0")]
    V1_0_0,
}
impl std::fmt::Display for SoccernetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
impl AsRef<str> for SoccernetVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::V1_0_0 => "1.0.0",
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum ClipsVersion {
    #[value(name = "1.0.0")]
    V1_0_0,
}
impl AsRef<str> for ClipsVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::V1_0_0 => "1.0.0",
        }
    }
}

impl std::fmt::Display for ClipsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Classifier {
    Soccernet {
        #[arg(short, long, default_value_t = SoccernetVersion::V1_0_0)]
        version: SoccernetVersion,
    },
    Clips {
        #[arg(short, long, default_value_t = ClipsVersion::V1_0_0)]
        version: ClipsVersion,
    },
}
impl std::fmt::Display for Classifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Soccernet { version } => write!(f, "SOCcerNET version: {}", version),
            Self::Clips { version } => write!(f, "CLIPS version: {}", version),
        }
    }
}

fn main() -> ExitCode {
    let args = Cli::parse();
    let config = match &args.classifier {
        Classifier::Soccernet { version } => {
            MODEL_CONFIG.get_config(&ModelType::SOCcerNET, version.as_ref())
        }
        Classifier::Clips { version } => {
            MODEL_CONFIG.get_config(&ModelType::CLIPS, version.as_ref())
        }
    };
    let Some(model_config) = config else {
        eprintln!("ERROR: Unknown model/version");
        return ExitCode::FAILURE;
    };

    match SoccerPipeline::build(model_config) {
        Ok(_) => {
            println!("models are cached");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            ExitCode::FAILURE
        }
    }
}
