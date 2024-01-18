use std::{
    net::ToSocketAddrs,
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
};

use snafu::ResultExt;

use crate::{
    context::Context,
    error::{self, Error},
    tunnel::{Tunnel, TunnelMeta, TunnelType},
};

pub struct DockerMount {
    pub host_endpoint: PathBuf,
    pub container_endpoint: PathBuf,
}

#[derive(Clone, Debug)]
pub struct DockerTunnel {
    pub meta: TunnelMeta,
    pub image_name: String,
    pub container_name: String,
    pub container_port: u16,
    pub listen_host: String,
    pub listen_port: u16,
}

impl DockerTunnel {
    pub fn start_with_mounts(
        &self,
        context: &Context,
        mounts: &[DockerMount],
    ) -> Result<(), Error> {
        let listen_addr = {
            let address = format!("{}:{}", self.listen_host, self.listen_port);
            address
                .to_socket_addrs()
                .with_context(|_| error::ResolveSocketAddrSnafu { address })?
                .next()
                .ok_or(Error::DomainNotFound { domain: self.listen_host.clone() })?
        };

        let mut args = vec![
            "run".to_owned(),
            "--detach".to_owned(),
            "--rm".to_owned(),
            "--name".to_owned(),
            self.container_name.clone(),
            "--publish".to_owned(),
            format!("{listen_addr}:{}", self.container_port),
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
                .spawn()
                .with_context(|_| error::SpawnDockerCommandSnafu)?
                .wait()
                .with_context(|_| error::WaitForDockerProcessSnafu)?,
        )
    }

    #[inline]
    fn convert_output(exit_status: ExitStatus) -> Result<(), Error> {
        match exit_status.code() {
            Some(0) | None => Ok(()),
            Some(code) => Err(Error::ExternalCommand { code }),
        }
    }
}

impl Tunnel for DockerTunnel {
    fn meta(&self) -> &TunnelMeta { &self.meta }

    #[inline]
    fn tunnel_type(&self) -> TunnelType { TunnelType::Docker }

    #[inline]
    fn start(&self, context: &Context) -> Result<(), Error> { self.start_with_mounts(context, &[]) }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> {
        if self.is_running(context)? {
            let exit_status = Command::new("docker")
                .args(["stop", &self.container_name])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .with_context(|_| error::SpawnDockerCommandSnafu)?
                .wait()
                .with_context(|_| error::WaitForDockerProcessSnafu)?;

            Self::convert_output(exit_status)
        } else {
            Ok(())
        }
    }

    #[inline]
    fn is_running(&self, _context: &Context) -> Result<bool, Error> {
        let output = Command::new("docker")
            .args(["inspect", "-f", "{{.State.Running}}", &self.container_name])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|_| error::SpawnDockerCommandSnafu)?
            .wait()
            .with_context(|_| error::WaitForDockerProcessSnafu)?;

        Ok(output.success())
    }
}
