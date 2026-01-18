use crate::config::Config;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct LocData {
    pub zh: String,
    pub en: String,
}

pub fn load_localisation(config: &Config) -> HashMap<String, LocData> {
    let zh_path = Path::new(config.stellaris_path())
        .join(config.localisation_dir())
        .join(config.zh_dir());

    let en_path = Path::new(config.stellaris_path())
        .join(config.localisation_dir())
        .join(config.en_dir());

    let (zh_map, mut en_map) = rayon::join(|| load_from_dir(&zh_path), || load_from_dir(&en_path));

    let mut result = HashMap::with_capacity(zh_map.len());
    for (key, zh) in zh_map {
        let en = en_map.remove(&key).unwrap_or_default();
        result.insert(key, LocData { zh, en });
    }

    result
}

fn load_from_dir(path: &Path) -> HashMap<String, String> {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "yml"))
        .collect();

    entries
        .par_iter()
        .map(|entry| process_file(entry.path()))
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                acc.extend(map);
                acc
            },
        )
}

fn process_file(path: &Path) -> HashMap<String, String> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };

    // Strip BOM if present
    let content = content.strip_prefix("\u{feff}").unwrap_or(&content);

    let yaml: serde_yaml_bw::Value = match serde_yaml_bw::from_str(content) {
        Ok(v) => v,
        Err(_) => return HashMap::new(),
    };

    let mut map = HashMap::new();

    if let serde_yaml_bw::Value::Mapping(root_map) = yaml {
        for (_, inner_value) in root_map {
            if let serde_yaml_bw::Value::Mapping(inner_map) = inner_value {
                for (k, v) in inner_map {
                    if let serde_yaml_bw::Value::String(key, _) = k {
                        let key = if let Some((base, suffix)) = key.rsplit_once(':') {
                            if !suffix.trim().is_empty()
                                && suffix.trim().chars().all(|c| c.is_ascii_digit())
                            {
                                base.trim().to_string()
                            } else {
                                key
                            }
                        } else {
                            key
                        };

                        let val_str = match v {
                            serde_yaml_bw::Value::String(s, _) => s,
                            serde_yaml_bw::Value::Number(n, _) => n.to_string(),
                            serde_yaml_bw::Value::Bool(b, _) => b.to_string(),
                            _ => continue,
                        };
                        map.insert(key, val_str);
                    }
                }
            }
        }
    }
    map
}
