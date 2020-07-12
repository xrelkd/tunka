use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use crate::{
    error::Error,
    tunnel::{self, DockerOpenVPNTunnel, DockerTunnel, SshTunnel, TunnelManager},
};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(tag = "type")]
enum Tunnel {
    #[serde(rename = "docker")]
    Docker {
        name: String,
        description: Option<String>,
        image_name: String,
        container_name: String,
        container_port: u16,
        listen_host: String,
        listen_port: u16,
    },
    #[serde(rename = "ssh")]
    Ssh {
        name: String,
        description: Option<String>,
        remote_host: String,
        remote_port: u16,
        user_name: String,
        identify_file: PathBuf,
        listen_host: String,
        listen_port: u16,
    },
    #[serde(rename = "docker-openvpn")]
    DockerOpenVPN {
        name: String,
        description: Option<String>,
        image_name: String,
        container_name: String,
        container_port: u16,
        listen_host: String,
        listen_port: u16,
        config_file: PathBuf,
    },
}

impl Into<Box<dyn tunnel::Tunnel>> for Tunnel {
    fn into(self) -> Box<dyn tunnel::Tunnel> {
        match self {
            Tunnel::Docker {
                name,
                description,
                image_name,
                container_name,
                container_port,
                listen_host,
                listen_port,
            } => Box::new(DockerTunnel::new(
                &name,
                description,
                &image_name,
                &container_name,
                container_port,
                &listen_host,
                listen_port,
            )),
            Tunnel::DockerOpenVPN {
                name,
                description,
                image_name,
                container_name,
                container_port,
                listen_host,
                listen_port,
                config_file,
            } => Box::new(DockerOpenVPNTunnel::new(
                &name,
                description,
                &image_name,
                &container_name,
                container_port,
                &listen_host,
                listen_port,
                config_file,
            )),
            Tunnel::Ssh {
                name,
                description,
                remote_host,
                remote_port,
                user_name,
                identify_file,
                listen_host,
                listen_port,
            } => Box::new(SshTunnel::new(
                &name,
                description,
                &remote_host,
                remote_port,
                &user_name,
                identify_file,
                &listen_host,
                listen_port,
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    control_path_directory: PathBuf,
    tunnels: Vec<Tunnel>,
}

impl Config {
    #[inline]
    pub fn from_str(s: &str) -> Result<Config, Error> {
        let config =
            serde_yaml::from_str(&s).map_err(|source| Error::ParseYamlConfig { source })?;
        Ok(config)
    }

    #[inline]
    pub fn from_file<P: AsRef<Path>>(config_file: P) -> Result<Config, Error> {
        let content = std::fs::read_to_string(&config_file).map_err(|source| {
            Error::ReadConfigFile { source, file_path: config_file.as_ref().to_owned() }
        })?;
        Self::from_str(&content)
    }

    #[inline]
    pub fn control_path_directory(&self) -> &Path { &self.control_path_directory }

    pub fn into_manager(self) -> TunnelManager {
        let tunnels = self.tunnels.into_iter().fold(BTreeMap::new(), |mut tunnels, tunnel| {
            let tunnel: Box<dyn tunnel::Tunnel> = tunnel.into();
            let tunnel_name = tunnel.name().to_owned();
            tunnels.insert(tunnel_name, tunnel);
            tunnels
        });

        TunnelManager { tunnels }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let data = r"
            control_path_directory: /tmp/tunka
            tunnels: []
            ";
        let config = Config::from_str(data).unwrap();
        assert_eq!(config.tunnels.is_empty(), true);
    }

    #[test]
    fn test_docker_tunnel() {
        let data = r"
            control_path_directory: /tmp/tunka
            tunnels:
                - type: docker
                  name: docker-tunnel
                  image_name: docker-tunnel
                  container_name: docker-tunnel
                  container_port: 8118
                  listen_host: 127.0.0.1
                  listen_port: 3128
            ";
        let config = Config::from_str(data).unwrap();
        assert_eq!(
            config.tunnels.first(),
            Some(&Tunnel::Docker {
                name: "docker-tunnel".to_owned(),
                description: None,
                image_name: "docker-tunnel".to_owned(),
                container_name: "docker-tunnel".to_owned(),
                container_port: 8118,
                listen_host: "127.0.0.1".to_owned(),
                listen_port: 3128,
            })
        )
    }

    #[test]
    fn test_ssh_tunnel() {
        let data = r"
            control_path_directory: /tmp/tunka
            tunnels:
                - type: ssh
                  name: ssh-tunnel
                  listen_host: 127.0.0.1
                  listen_port: 8051
                  remote_host: www.google.com
                  remote_port: 26
                  user_name: the-user
                  identify_file: /tmp/id
            ";
        let config = Config::from_str(data).unwrap();
        assert_eq!(
            config.tunnels.first(),
            Some(&Tunnel::Ssh {
                name: "ssh-tunnel".to_owned(),
                description: None,
                listen_host: "127.0.0.1".to_owned(),
                listen_port: 8051,
                remote_host: "www.google.com".to_owned(),
                remote_port: 26,
                user_name: "the-user".to_owned(),
                identify_file: "/tmp/id".into(),
            })
        )
    }
}
