use std::env;
use std::str::FromStr;

/// Get an environment variable or return a default value
pub fn env_or_default<T>(key: &str, default: T) -> T
where
    T: FromStr,
{
    env::var(key)
        .ok()
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or(default)
}
