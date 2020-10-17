/// Represents an error that is encountered while loading and parsing a
/// specification from a `Polyhorn.toml` file.
#[derive(Debug)]
pub enum Error {
    /// Contains an error that is encountered while loading the contents of the
    /// `Polyhorn.toml` file from disk.
    IO(std::io::Error),

    /// Contains an error that is encountered while parsing the `Polyhorn.toml`
    /// file in memory.
    TOML(toml::de::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Error::TOML(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
