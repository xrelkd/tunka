use std::collections::BTreeMap;

use crate::context::Context;
use crate::error::Error;

mod docker;
mod ssh;

pub use self::docker::DockerTunnel;
pub use self::ssh::SshTunnel;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TunnelMeta {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TunnelType {
    Ssh,
    Docker,
}

impl std::fmt::Display for TunnelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TunnelType::Ssh => write!(f, "SSH tunnel"),
            TunnelType::Docker => write!(f, "Docker Tunnel"),
        }
    }
}

pub trait Tunnel {
    fn name(&self) -> &str;

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
    pub fn list(&self) -> Vec<String> {
        self.tunnels.keys().map(ToOwned::to_owned).collect()
    }

    #[inline]
    pub fn metadata_list(&self) -> Vec<TunnelMeta> {
        self.tunnels.values().map(|t| t.meta().clone()).collect()
    }

    #[inline]
    pub fn start(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        std::fs::create_dir_all(context.control_path_directory())?;

        let tunnel = self.get_tunnel(tunnel_name)?;
        println!("Start {} {}", tunnel.tunnel_type(), tunnel_name);

        tunnel.start(context)?;
        self.log_running_status(context, tunnel_name)?;
        Ok(())
    }

    #[inline]
    pub fn stop(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        let tunnel = self.get_tunnel(tunnel_name)?;
        if tunnel.is_running(context)? {
            info!("Stop {} {}", tunnel.tunnel_type(), tunnel_name);
            tunnel.stop(context)?;
        }

        self.log_running_status(context, tunnel_name)?;
        Ok(())
    }

    #[inline]
    pub fn restart(&self, context: &Context, tunnel_name: &str) -> Result<(), Error> {
        self.get_tunnel(tunnel_name)?.restart(context)
    }

    #[inline]
    pub fn log_running_status(&self, context: &Context, tunnel_name: &str) -> Result<bool, Error> {
        if self.is_running(context, tunnel_name)? {
            info!("{} is running", tunnel_name);
            Ok(true)
        } else {
            info!("{} is not running", tunnel_name);
            Ok(false)
        }
    }

    #[inline]
    pub fn is_running(&self, context: &Context, tunnel_name: &str) -> Result<bool, Error> {
        self.get_tunnel(tunnel_name)?.is_running(context)
    }

    #[inline]
    pub fn get_tunnel(&self, tunnel_name: &str) -> Result<&Box<dyn Tunnel>, Error> {
        match self.tunnels.get(tunnel_name) {
            Some(tunnel) => Ok(tunnel),
            None => Err(Error::TunnelNotFound(tunnel_name.to_owned())),
        }
    }

    #[inline]
    pub fn start_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().map(|t| self.start(context, t)).collect()
    }

    #[inline]
    pub fn stop_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().map(|t| self.stop(context, t)).collect()
    }

    #[inline]
    pub fn restart_all(&self, context: &Context) -> Result<(), Error> {
        self.list().iter().map(|t| self.restart(context, t)).collect()
    }
}
