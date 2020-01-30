use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

use crate::context::Context;
use crate::error::Error;
use crate::tunnel::{Tunnel, TunnelMeta, TunnelType};

pub struct DockerMount {
    pub host_endpoint: PathBuf,
    pub container_endpoint: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DockerTunnel {
    meta: TunnelMeta,
    image_name: String,
    container_name: String,
    container_port: u16,
    listen_host: String,
    listen_port: u16,
}

impl DockerTunnel {
    pub fn new(
        name: &str,
        description: Option<String>,
        image_name: &str,
        container_name: &str,
        container_port: u16,
        listen_host: &str,
        listen_port: u16,
    ) -> DockerTunnel {
        let meta = TunnelMeta { name: name.to_owned(), description };

        DockerTunnel {
            meta,
            image_name: image_name.to_owned(),
            container_name: container_name.to_owned(),
            container_port,
            listen_host: listen_host.to_owned(),
            listen_port,
        }
    }

    pub fn start_with_mounts(
        &self,
        context: &Context,
        mounts: &[DockerMount],
    ) -> Result<(), Error> {
        use std::net::ToSocketAddrs;
        let listen_addr =
            match format!("{}:{}", self.listen_host, self.listen_port).to_socket_addrs()?.next() {
                Some(addr) => addr,
                None => return Err(Error::DomainNotFound(self.listen_host.clone())),
            };

        let mut args = vec![
            "run".to_owned(),
            "--detach".to_owned(),
            "--rm".to_owned(),
            "--name".to_owned(),
            self.container_name.clone(),
            "--publish".to_owned(),
            format!("{}:{}", listen_addr, self.container_port),
            "--device=/dev/net/tun".to_owned(),
            "--cap-add=NET_ADMIN".to_owned(),
        ];

        for mount in mounts {
            args.push("--mount".to_owned());
            args.push(format!(
                "type=bind,source={},destination={},readonly=true",
                context.apply_path(&mount.host_endpoint).to_string_lossy(),
                mount.container_endpoint.to_string_lossy(),
            ));
        }

        args.push(self.image_name.clone());

        Self::convert_output(
            Command::new("docker")
                .args(&args)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?
                .wait()?,
        )
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
    fn start(&self, context: &Context) -> Result<(), Error> {
        self.start_with_mounts(context, &[])
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
            .args(&["inspect", "-f", "{{.State.Running}}", &self.container_name])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;
        Ok(output.success())
    }
}
