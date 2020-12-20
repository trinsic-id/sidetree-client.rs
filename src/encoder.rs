use base64::{Config, URL_SAFE};

pub fn encode<T: AsRef<[u8]>>(value: T) -> String {
    ::base64::encode_config(value, URL_SAFE)
}

pub fn encode_config<T: AsRef<[u8]>>(value: T, config: Config) -> String {
    ::base64::encode_config(value, config)
}
