use clap::{Parser, ValueEnum};

/// Supported languages
#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Zh,
    En,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Zh => "zh",
            Language::En => "en",
        }
    }
}

/// Stellaris Tech Tree Parser
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Path to the Stellaris installation directory
    #[arg(
        short,
        long,
        default_value = "C:/Software/Steam/steamapps/common/Stellaris"
    )]
    stellaris_path: String,

    /// Path to the mod directory
    #[arg(
        short,
        long,
        default_value = "C:/Users/$UserName/Documents/Paradox Interactive/Stellaris/mod"
    )]

    /// Path to the resource directory
    #[arg(short, long, default_value = "res")]
    resource_path: String,

    /// Path to the output directory
    #[arg(short, long, default_value = "output")]
    output_path: String,

    /// Language of the output files
    #[arg(short, long, value_enum, default_value_t = Language::Zh)]
    language: Language,

    /// Whether to convert GFX files
    #[arg(short, long, default_value_t = false)]
    convert_gfx: bool,
}

impl Args {
    pub fn stellaris_path(&self) -> &str {
        &self.stellaris_path
    }

    pub fn resource_path(&self) -> &str {
        &self.resource_path
    }

    pub fn output_path(&self) -> &str {
        &self.output_path
    }

    pub fn language(&self) -> &str {
        self.language.as_str()
    }

    pub fn convert_gfx(&self) -> bool {
        self.convert_gfx
    }
}

pub fn get_args() -> Args {
    Args::parse()
}
