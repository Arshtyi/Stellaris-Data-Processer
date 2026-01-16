use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn convert_dds_to_png(input_dir: &Path, output_dir: &Path) {
    if !input_dir.exists() {
        eprintln!("Directory not found: {:?}", input_dir);
        return;
    }

    println!("Scanning for DDS files in: {:?}", input_dir);

    let mut total_count = 0;
    let mut success_count = 0;

    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Avoid processing output directory if it's inside input directory
        if path.starts_with(output_dir) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy().eq_ignore_ascii_case("dds") {
                    total_count += 1;
                    if let Ok(relative) = path.strip_prefix(input_dir) {
                        let output_file = output_dir.join(relative).with_extension("png");
                        if let Some(parent) = output_file.parent() {
                            if !parent.exists() {
                                if let Err(e) = fs::create_dir_all(parent) {
                                    eprintln!("Failed to create directory {:?}: {}", parent, e);
                                    continue;
                                }
                            }
                        }
                        let mut converted = false;
                        if let Ok(file) = std::fs::File::open(path) {
                            let mut reader = std::io::BufReader::new(&file);
                            if let Ok(dds) = ddsfile::Dds::read(&mut reader) {
                                if let Ok(img) = image_dds::image_from_dds(&dds, 0) {
                                    match img.save(&output_file) {
                                        Ok(_) => {
                                            success_count += 1;
                                            converted = true;
                                            // if success_count % 100 == 0 {
                                            //     println!("Processed {} files...", success_count);
                                            // }
                                        }
                                        Err(e) => eprintln!(
                                            "Failed to save PNG for {:?}: {}",
                                            relative, e
                                        ),
                                    }
                                } else {
                                    eprintln!("Failed to decode DDS content: {:?}", relative);
                                }
                            } else {
                                eprintln!("Failed to parse DDS header: {:?}", relative);
                            }
                        } else {
                            eprintln!("Failed to open file: {:?}", path);
                        }

                        if !converted {
                            eprintln!("Failed to convert DDS: {:?}", relative);
                        }
                    }
                }
            }
        }
    }

    println!(
        "Conversion finished. {}/{} files successfully converted.",
        success_count, total_count
    );
}
