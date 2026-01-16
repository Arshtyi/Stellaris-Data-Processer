use crate::helper;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct YamlConfig {
    common_dir: String,
    events_dir: String,
    localisation_dir: String,
    gfx_dir: String,
    en_dir: String,
    zh_dir: String,
    output_gfx_dir: String,
}

pub struct Config {
    stellaris_path: String,
    resource_path: String,
    output_path: String,
    language: String,
    common_dir: String,
    events_dir: String,
    localisation_dir: String,
    gfx_dir: String,
    en_dir: String,
    zh_dir: String,
    output_gfx_dir: String,
}

impl Config {
    pub fn new(
        stellaris_path: String,
        resource_path: String,
        output_path: String,
        language: String,
    ) -> Self {
        let config_path = std::path::Path::new(&resource_path).join("config.yml");
        let content = std::fs::read_to_string(&config_path).unwrap_or_else(|e| {
            helper::safely_exit(
                &format!(
                    "Failed to read config file {}: {}",
                    config_path.display(),
                    e
                ),
                1,
            );
        });

        let yaml_config: YamlConfig = serde_yaml_bw::from_str(&content).unwrap_or_else(|e| {
            helper::safely_exit(&format!("Failed to parse config file: {}", e), 1);
        });

        Config {
            stellaris_path,
            resource_path,
            output_path,
            language,
            common_dir: yaml_config.common_dir,
            events_dir: yaml_config.events_dir,
            localisation_dir: yaml_config.localisation_dir,
            gfx_dir: yaml_config.gfx_dir,
            en_dir: yaml_config.en_dir,
            zh_dir: yaml_config.zh_dir,
            output_gfx_dir: yaml_config.output_gfx_dir,
        }
    }

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
        &self.language
    }

    pub fn common_dir(&self) -> &str {
        &self.common_dir
    }

    pub fn events_dir(&self) -> &str {
        &self.events_dir
    }

    pub fn localisation_dir(&self) -> &str {
        &self.localisation_dir
    }

    pub fn gfx_dir(&self) -> &str {
        &self.gfx_dir
    }

    pub fn en_dir(&self) -> &str {
        &self.en_dir
    }

    pub fn zh_dir(&self) -> &str {
        &self.zh_dir
    }

    pub fn output_gfx_dir(&self) -> &str {
        &self.output_gfx_dir
    }

    pub fn display(&self) {
        println!("Basic Configuration:");
        println!("  Stellaris Path: {}", self.stellaris_path);
        println!("  Resource Path: {}", self.resource_path);
        println!("  Output Path: {}", self.output_path);
        println!("  Language: {}", self.language);
        // println!("  Common Dir: {}", self.common_dir);
        // println!("  Events Dir: {}", self.events_dir);
        // println!("  Localisation Dir: {}", self.localisation_dir);
        // println!("  Gfx Dir: {}", self.gfx_dir);
        // println!("  En Dir: {}", self.en_dir);
        // println!("  Zh Dir: {}", self.zh_dir);
        // println!("  Output Gfx Dir: {}", self.output_gfx_dir);
    }
}
