use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use soccer_cache::Cache;

use crate::SoccernetVersion::V1_0_0;

trait ModelInfo {
    fn get_soccer_model(&self) -> &'static str;
    fn get_soccer_url(&self) -> String {
        format!(
            "https://danielruss.github.io/soccer-models/{}",
            self.get_soccer_model()
        )
    }
    fn get_embedding_model(&self) -> &'static str;
}

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
    classifier: Option<Classifier>,

    #[arg(long)]
    clear: bool,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum SoccernetVersion {
    #[value(name = "1.0.0")]
    V1_0_0,
}

impl ModelInfo for SoccernetVersion {
    fn get_soccer_model(&self) -> &'static str {
        match self {
            Self::V1_0_0 => "SOCcer_v3.0.0.onnx",
        }
    }
    fn get_embedding_model(&self) -> &'static str {
        match self {
            Self::V1_0_0 => "Xenova/GIST-small-Embedding-v0",
        }
    }
}
impl AsRef<str> for SoccernetVersion {
    fn as_ref(&self) -> &str {
        match self {
            Self::V1_0_0 => "1.0.0",
        }
    }
}
impl std::fmt::Display for SoccernetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum ClipsVersion {
    #[value(name = "1.0.0")]
    V1_0_0,
}
impl ModelInfo for ClipsVersion {
    fn get_soccer_model(&self) -> &'static str {
        match self {
            Self::V1_0_0 => "clips_v0.0.2.onnx",
        }
    }
    fn get_embedding_model(&self) -> &'static str {
        match self {
            Self::V1_0_0 => "Xenova/GIST-small-Embedding-v0",
        }
    }
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

    if args.clear {
        return match Cache::clear_cache("Xenova/GIST-small-Embedding-v0") {
            Ok(()) => {
                println!("model cache cleared");
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("could not clear model cache: {error}");
                ExitCode::FAILURE
            }
        };
    }

    let cache = Cache::new();
    let cache = match cache {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };

    let classifier = args
        .classifier
        .unwrap_or(Classifier::Soccernet { version: V1_0_0 });

    match &classifier {
        Classifier::Soccernet { version } => {
            let model_id = version.get_embedding_model();

            let embed_cached = Cache::is_cached(model_id);
            let soccer_cached = Cache::is_soccer_cached(version.get_soccer_model());

            if soccer_cached && embed_cached {
                println!("{} is already cached", classifier);
                return ExitCode::SUCCESS;
            }

            if !soccer_cached {
                let cached_pathbuf_res = cache.get_from_url(&version.get_soccer_url());
                if let Err(error) = cached_pathbuf_res {
                    eprintln!("Problem caching SOCcerNET: {error}");
                    return ExitCode::FAILURE;
                }
            }
            if !embed_cached {
                let cached_pathbuf_res = cache.cache_hf_model(model_id);
                if let Err(error) = cached_pathbuf_res {
                    eprintln!("Problem caching {} from Hugging Face: {error}", model_id);
                    return ExitCode::FAILURE;
                }
            }
        }
        Classifier::Clips { version } => {
            let model_id = version.get_embedding_model();

            let embed_cached = Cache::is_cached(model_id);
            let soccer_cached = Cache::is_soccer_cached(version.get_soccer_model());

            if soccer_cached && embed_cached {
                println!("{} is already cached", classifier);
                return ExitCode::SUCCESS;
            }
            if !soccer_cached {
                let cached_pathbuf_res = cache.get_from_url(&version.get_soccer_url());
                if let Err(error) = cached_pathbuf_res {
                    eprintln!("Problem caching CLIPS: {error}");
                    return ExitCode::FAILURE;
                }
            }
            if !embed_cached {
                let cached_pathbuf_res = cache.cache_hf_model(model_id);
                if let Err(error) = cached_pathbuf_res {
                    eprintln!("Problem caching {} from Hugging Face: {error}", model_id);
                    return ExitCode::FAILURE;
                }
            }
        }
    };

    ExitCode::SUCCESS
}
