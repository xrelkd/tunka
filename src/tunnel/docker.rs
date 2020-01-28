use std::net::SocketAddr;
use std::process::{Command, ExitStatus, Stdio};

use crate::context::Context;
use crate::error::Error;
use crate::tunnel::{Tunnel, TunnelMeta, TunnelType};

#[derive(Debug, Clone)]
pub struct DockerTunnel {
    meta: TunnelMeta,
    image_name: String,
    container_name: String,
    container_port: u16,
    listen_addr: SocketAddr,
}

impl DockerTunnel {
    pub fn new(
        name: &str,
        description: Option<String>,
        image_name: &str,
        container_name: &str,
        container_port: u16,
        listen_addr: SocketAddr,
    ) -> DockerTunnel {
        let meta = TunnelMeta { name: name.to_owned(), description };

        DockerTunnel {
            meta,
            image_name: image_name.to_owned(),
            container_name: container_name.to_owned(),
            container_port,
            listen_addr,
        }
    }

    #[inline]
    fn convert_output(exit_status: ExitStatus) -> Result<(), Error> {
        match exit_status.code() {
            Some(0) | None => Ok(()),
            Some(code) => Err(Error::ExternalCommand(code)),
        }
    }
}

impl Tunnel for DockerTunnel {
    #[inline]
    fn name(&self) -> &str {
        &self.meta.name
    }

    fn meta(&self) -> &TunnelMeta {
        &self.meta
    }

    #[inline]
    fn tunnel_type(&self) -> TunnelType {
        TunnelType::Docker
    }

    #[inline]
    fn start(&self, _context: &Context) -> Result<(), Error> {
        Self::convert_output(
            Command::new("docker")
                .args(&[
                    "run",
                    "--detach",
                    "--rm",
                    "--name",
                    &self.container_name,
                    "--publish",
                    &format!(
                        "{}:{}:{}",
                        self.listen_addr.ip(),
                        self.listen_addr.port(),
                        self.container_port
                    ),
                    "--device=/dev/net/tun",
                    "--cap-add=NET_ADMIN",
                    &self.image_name,
                ])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?
                .wait()?,
        )
    }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> {
        if self.is_running(context)? {
            let exit_status = Command::new("docker")
                .args(&["stop", &self.container_name])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?
                .wait()?;
            return Self::convert_output(exit_status);
        }
        Ok(())
    }

    #[inline]
    fn is_running(&self, _context: &Context) -> Result<bool, Error> {
        let output = Command::new("docker")
            .args(&["inspect", "-f", "{{.State.Running}}", &self.image_name])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(output.success())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::thread::sleep;
//     use std::time::Duration;

//     #[test]
//     fn i() {
//         let tunnel = DockerTunnel::new(
//             "fstk-tunnel",
//             "fstk-tunnel",
//             "fstk-tunnel",
//             8118,
//             "127.0.0.1:3200".parse().unwrap(),
//         );

//         assert_eq!(tunnel.is_running().unwrap(), false);

//         let _ = tunnel.start();
//         sleep(Duration::from_secs(10));

//         assert_eq!(tunnel.is_running().unwrap(), true);

//         let _ = tunnel.stop();
//         sleep(Duration::from_secs(10));
//         assert_eq!(tunnel.is_running().unwrap(), false);
//     }
// }
