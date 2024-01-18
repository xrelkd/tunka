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
    pub auth_file: Option<PathBuf>,
}

impl Tunnel for DockerOpenVPNTunnel {
    #[inline]
    fn meta(&self) -> &TunnelMeta { self.docker_tunnel.meta() }

    #[inline]
    fn tunnel_type(&self) -> TunnelType { TunnelType::DockerOpenVPN }

    #[inline]
    fn start(&self, context: &Context) -> Result<(), Error> {
        let mounts = {
            let mut m = vec![DockerMount {
                host_endpoint: self.config_file.clone(),
                container_endpoint: PathBuf::from("/config.ovpn"),
            }];

            if let Some(auth_file) = &self.auth_file {
                m.push(DockerMount {
                    host_endpoint: auth_file.clone(),
                    container_endpoint: PathBuf::from("/auth.txt"),
                });
            }

            m
        };

        self.docker_tunnel.start_with_mounts(context, &mounts)
    }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> { self.docker_tunnel.stop(context) }

    #[inline]
    fn is_running(&self, context: &Context) -> Result<bool, Error> {
        self.docker_tunnel.is_running(context)
    }
}
