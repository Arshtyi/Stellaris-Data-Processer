mod cli;
mod config;
mod core;
mod helper;

use std::path::Path;

fn main() {
    let args = cli::get_args();
    let config = config::Config::new(
        match helper::resolve_path(args.stellaris_path()) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(e) => {
                helper::safely_exit(&format!("Error resolving Stellaris path: {}", e), 1);
            }
        },
        match helper::resolve_path(args.resource_path()) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(e) => {
                helper::safely_exit(&format!("Error resolving resource path: {}", e), 1);
            }
        },
        match helper::resolve_path_and_create(args.output_path()) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(e) => {
                helper::safely_exit(&format!("Error resolving/creating output path: {}", e), 1);
            }
        },
        args.language().to_string(),
    );
    config.display();

    // let stellaris_gfx_path = Path::new(config.stellaris_path()).join(config.gfx_dir());
    // let output_gfx_path = Path::new(config.output_path()).join(config.output_gfx_dir());

    // println!("Stellaris GFX Path: {:?}", stellaris_gfx_path);
    // println!("Output GFX Path: {:?}", output_gfx_path);

    // core::image::convert_dds_to_png(&stellaris_gfx_path, &output_gfx_path);
}
