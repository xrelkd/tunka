use std::path::PathBuf;

use crate::{
    context::Context,
    error::Error,
    tunnel::{docker::DockerMount, DockerTunnel, Tunnel, TunnelMeta, TunnelType},
};

#[derive(Debug, Clone)]
pub struct DockerOpenVPNTunnel {
    pub docker_tunnel: DockerTunnel,
    pub config_file: PathBuf,
}

impl Tunnel for DockerOpenVPNTunnel {
    #[inline]
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
