use std::path::PathBuf;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read configuration file {}, error: {}", file_path.display(), source))]
    ReadConfigFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not create control path directory {}, error: {}", dir_path.display(), source))]
    CreateControlPathDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("Domain not found: {}", domain))]
    DomainNotFound { domain: String },

    #[snafu(display("Tunnel not found: {}", tunnel))]
    TunnelNotFound { tunnel: String },

    #[snafu(display("External command error, exit code: {}", code))]
    ExternalCommand { code: i32 },

    #[snafu(display("User name not found"))]
    UserNameNotFound,

    #[snafu(display("Home directory not found"))]
    HomeDirectoryNotFound,

    #[snafu(display("User's configuration directory not found"))]
    UserConfigDirectoryNotFound,

    #[snafu(display("Could not resolve socket address {}, error: {}", addr, source))]
    ResolveSocketAddr { addr: String, source: std::io::Error },

    #[snafu(display("Failed to parse YAML, error: {}", source))]
    ParseYamlConfig { source: serde_yaml::Error },

    #[snafu(display("Error occurred while spawning SSH command, error: {}", source))]
    SpawnSshCommand { source: std::io::Error },

    #[snafu(display("Error occurred while waiting for SSH process, error: {}", source))]
    WaitForSshProcess { source: std::io::Error },

    #[snafu(display("Error occurred while spawning Docker command, error: {}", source))]
    SpawnDockerCommand { source: std::io::Error },

    #[snafu(display("Error occurred while waiting for Docker process, error: {}", source))]
    WaitForDockerProcess { source: std::io::Error },
}
