#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    StdIo(#[cause] std::io::Error),

    #[fail(display = "tunnel not found: {}", _0)]
    TunnelNotFound(String),

    #[fail(display = "external command error, exit code: {}", _0)]
    ExternalCommand(i32),

    #[fail(display = "user name not found")]
    UserNameNotFound,

    #[fail(display = "home directory not found")]
    HomeDirectoryNotFound,

    #[fail(display = "failed to parse YAML {}", _0)]
    SerdeYaml(serde_yaml::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::StdIo(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::SerdeYaml(err)
    }
}
