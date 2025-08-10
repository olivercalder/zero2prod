#[derive(serde::Deserialize)]
pub struct Settings {
    pub database_path: String,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?
        .try_deserialize::<Settings>()
}
