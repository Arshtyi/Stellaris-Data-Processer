use clap::Parser;

/// Stellaris Tech Tree Parser
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Path to the Stellaris installation directory
    #[arg(long, default_value = "C:/Software/Steam/steamapps/common/Stellaris")]
    stellaris_path: String,

    /// Path to the resource directory
    #[arg(long, default_value = "res")]
    resource_path: String,

    /// Path to the output directory
    #[arg(long, default_value = "output")]
    output_path: String,

    /// Whether to convert GFX files
    #[arg(long, default_value_t = false)]
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

    pub fn convert_gfx(&self) -> bool {
        self.convert_gfx
    }
}
pub fn get_args() -> Args {
    Args::parse()
}
