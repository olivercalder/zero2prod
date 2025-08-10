#[derive(serde::Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DatabaseSettings {
    Sqlite(SqliteSettings),
    Postgres(PostgresSettings),
}

#[derive(serde::Deserialize, Clone)]
pub struct SqliteSettings {
    pub path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct PostgresSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        match self {
            DatabaseSettings::Sqlite(s) => format!("sqlite://{}", s.path),
            DatabaseSettings::Postgres(s) => format!(
                "postgres://{}:{}@{}:{}/{}",
                s.username, s.password, s.host, s.port, s.database_name
            ),
        }
    }
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
