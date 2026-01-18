use crate::config::Config;
use crate::core::localisation::LocData;
use jomini::TextTape;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Achievement {
    pub id: String,
    pub en_name: String,
    pub en_desc: String,
    pub zh_name: String,
    pub zh_desc: String,
    pub key: String,
    pub displayname: String,
    pub name: String,
    pub version: String,
    pub category: String,
    // possible_logic: TBD
    // happened_logic: TBD
}

pub fn process_achievements(
    config: &Config,
    loc_map: &HashMap<String, LocData>,
) -> HashMap<String, Achievement> {
    let mut achievements = HashMap::new();
    let common_path = Path::new(config.stellaris_path()).join(config.common_dir());
    let version = config.game_version().to_string();

    for file_name in config.achievements() {
        let file_path = common_path.join(file_name);
        if !file_path.exists() {
            eprintln!("Achievement file not found: {:?}", file_path);
            continue;
        }

        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to read file {:?}: {}", file_path, e);
                continue;
            }
        };

        // Strip BOM if present
        let content = content.strip_prefix("\u{feff}").unwrap_or(&content);

        let tape = match TextTape::from_slice(content.as_bytes()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to parse file {:?}: {}", file_path, e);
                continue;
            }
        };

        let root_reader = tape.utf8_reader();

        for (key, _op, value) in root_reader.fields() {
            let key_str = key.read_str();

            // Each value should be an object containing "id"
            if let Ok(obj_reader) = value.read_object() {
                let mut id_val: Option<u32> = None;

                for (field_key, _op, field_value) in obj_reader.fields() {
                    if field_key.read_str() == "id" {
                        if let Ok(scalar) = field_value.read_scalar() {
                            if let Ok(id) = scalar.to_u64() {
                                id_val = Some(id as u32);
                            }
                        }
                    }
                }

                if let Some(id) = id_val {
                    // Calculate x and y
                    // id=(x-2)*32+y+1 -> id-1 = (x-2)*32 + y
                    let val = id.saturating_sub(1);
                    let y = val % 32;
                    let x = (val / 32) + 2;

                    let name_key = format!("NEW_ACHIEVEMENT_{}_{}_NAME", x, y);
                    let desc_key = format!("NEW_ACHIEVEMENT_{}_{}_DESC", x, y);

                    // Lookup localization
                    let (zh_name, en_name) = match loc_map.get(&name_key) {
                        Some(data) => (data.zh.clone(), data.en.clone()),
                        None => continue, // User requirement: skip if not found (implied by "if not found zh, skip")
                    };

                    let (zh_desc, en_desc) = match loc_map.get(&desc_key) {
                        Some(data) => (data.zh.clone(), data.en.clone()),
                        None => continue,
                    };

                    // Extra check: passed only if zh is present?
                    // LocData usually has empty string if missing?
                    // `load_localisation` implementation: zh is main key, en is optional.
                    // If key exists in `loc_map`, zh is guaranteed to be present (it iterates zh_map).
                    // So simple existence check is enough.

                    let achievement = Achievement {
                        id: id.to_string(),
                        en_name: en_name.clone(),
                        en_desc,
                        zh_name: zh_name.clone(),
                        zh_desc,
                        key: key_str.to_string(),
                        displayname: zh_name,
                        name: en_name,
                        version: version.clone(),
                        category: "achievements".to_string(),
                    };

                    achievements.insert(key_str.to_string(), achievement);
                }
            }
        }
    }

    achievements
}
