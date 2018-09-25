
use toml;
use serde::de;

//use ws::Settings;

pub fn parse_config_from_buffer<'de, T>(s: &'de str) -> Result<T, toml::de::Error>
where
    T: de::Deserialize<'de>,
{
    Ok(toml::from_str::<T>(s)?)
}

macro_rules! parse_config {
    ($type: ty, $path: expr) => {{
        use std::fs::File;
        use std::io::Read;

        let mut buffer = String::new();

        File::open($path)
            .and_then(|mut f| f.read_to_string(&mut buffer))
            .unwrap_or_else(|err| panic!("Error while loading config: [{}]", err));

        parse_config_from_buffer::<$type>(&buffer)
            .unwrap_or_else(|err| panic!("Error while parsing config: [{}]", err))
    }}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub http_config: HttpConfig,
}

impl Config {
    pub fn new(path: &str) -> Self {
        parse_config!(Config, path)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub enable : bool,
    pub thread_number : Option<usize>,
    pub listen_ip : String,
    pub listen_port : String,
    pub timeout : u64,
    pub allow_origin : Option<String>,
}
