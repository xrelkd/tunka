use std::path::{Path, PathBuf};

use crate::{
    context::Context,
    error::Error,
    tunnel::{docker::DockerMount, DockerTunnel, Tunnel, TunnelMeta, TunnelType},
};

#[derive(Debug, Clone)]
pub struct DockerOpenVPNTunnel {
    docker_tunnel: DockerTunnel,
    config_file: PathBuf,
}

impl DockerOpenVPNTunnel {
    pub fn new<P: AsRef<Path>>(
        name: &str,
        description: Option<String>,
        image_name: &str,
        container_name: &str,
        container_port: u16,
        listen_host: &str,
        listen_port: u16,
        config_file: P,
    ) -> DockerOpenVPNTunnel {
        let docker_tunnel = DockerTunnel::new(
            name,
            description,
            image_name,
            container_name,
            container_port,
            listen_host,
            listen_port,
        );
        DockerOpenVPNTunnel { docker_tunnel, config_file: config_file.as_ref().to_owned() }
    }
}

impl Tunnel for DockerOpenVPNTunnel {
    #[inline]
    fn name(&self) -> &str { self.docker_tunnel.name() }

    fn meta(&self) -> &TunnelMeta { self.docker_tunnel.meta() }

    #[inline]
    fn tunnel_type(&self) -> TunnelType { TunnelType::DockerOpenVPN }

    #[inline]
    fn start(&self, context: &Context) -> Result<(), Error> {
        self.docker_tunnel.start_with_mounts(
            context,
            &[DockerMount {
                host_endpoint: self.config_file.clone(),
                container_endpoint: PathBuf::from("/config.ovpn"),
            }],
        )
    }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> { self.docker_tunnel.stop(context) }

    #[inline]
    fn is_running(&self, context: &Context) -> Result<bool, Error> {
        self.docker_tunnel.is_running(context)
    }
}
