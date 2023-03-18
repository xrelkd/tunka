use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Serialize};

use crate::{context::Context, error::Error};

mod docker;
mod docker_openvpn;
mod ssh;

pub use self::{docker::DockerTunnel, docker_openvpn::DockerOpenVPNTunnel, ssh::SshTunnel};

#[derive(Debug, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TunnelMeta {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TunnelType {
    Ssh,
    Docker,
    DockerOpenVPN,
}

impl fmt::Display for TunnelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TunnelType::Ssh => write!(f, "SSH tunnel"),
            TunnelType::Docker => write!(f, "Docker Tunnel"),
            TunnelType::DockerOpenVPN => write!(f, "Docker OpenVPN Tunnel"),
        }
    }
}

pub trait Tunnel {
    fn name(&self) -> &str { &self.meta().name }

    fn meta(&self) -> &TunnelMeta;

    fn tunnel_type(&self) -> TunnelType;

    fn start(&self, context: &Context) -> Result<(), Error>;

    fn stop(&self, context: &Context) -> Result<(), Error>;

    fn restart(&self, context: &Context) -> Result<(), Error> {
        self.stop(context)?;
        self.start(context)
    }

    fn is_running(&self, context: &Context) -> Result<bool, Error>;
}

pub struct TunnelManager {
    pub tunnels: BTreeMap<String, Box<dyn Tunnel>>,
}

impl TunnelManager {
    #[inline]
    pub fn list(&self) -> Vec<String> { self.tunnels.keys().map(ToOwned::to_owned).collect() }

    #[inline]
    pub fn metadata_list(&self) -> Vec<TunnelMeta> {
        self.tunnels.values().map(|t| t.meta().clone()).collect()
    }

    #[inline]
    pub fn start(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        let dir_path = context.control_path_directory();
        std::fs::create_dir_all(&dir_path)
            .map_err(|source| Error::CreateControlPathDirectory { source, dir_path })?;

        let tunnel = self
            .tunnels
            .get(tunnel_name)
            .ok_or(Error::TunnelNotFound { tunnel: tunnel_name.to_owned() })?;
        println!("Start {} {tunnel_name}", tunnel.tunnel_type());

        tunnel.start(context)?;
        self.log_running_status(context, tunnel_name)?;
        Ok(())
    }

    #[inline]
    pub fn stop(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        let tunnel = self
            .tunnels
            .get(tunnel_name)
            .ok_or(Error::TunnelNotFound { tunnel: tunnel_name.to_owned() })?;

        if tunnel.is_running(context)? {
            tracing::info!("Stop {} {tunnel_name}", tunnel.tunnel_type());
            tunnel.stop(context)?;
        }

        self.log_running_status(context, tunnel_name)?;
        Ok(())
    }

    #[inline]
    pub fn restart(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        self.tunnels
            .get(tunnel_name)
            .ok_or(Error::TunnelNotFound { tunnel: tunnel_name.to_owned() })?
            .restart(context)
    }

    #[inline]
    pub fn log_running_status(&self, context: &Context, tunnel_name: &str) -> Result<bool, Error> {
        if self.is_running(context, tunnel_name)? {
            tracing::info!("{tunnel_name} is running");
            Ok(true)
        } else {
            tracing::info!("{tunnel_name} is not running");
            Ok(false)
        }
    }

    #[inline]
    pub fn is_running(&self, context: &Context, tunnel_name: &str) -> Result<bool, Error> {
        self.tunnels
            .get(tunnel_name)
            .ok_or(Error::TunnelNotFound { tunnel: tunnel_name.to_owned() })?
            .is_running(context)
    }

    #[inline]
    pub fn start_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().try_for_each(|t| self.start(context, t))
    }

    #[inline]
    pub fn stop_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().try_for_each(|t| self.stop(context, t))
    }

    #[inline]
    pub fn restart_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().try_for_each(|t| self.restart(context, t))
    }
}
