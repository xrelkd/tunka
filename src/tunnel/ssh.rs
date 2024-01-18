use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use snafu::ResultExt;

use crate::{
    context::Context,
    error,
    error::Error,
    tunnel::{Tunnel, TunnelMeta, TunnelType},
};

#[derive(Clone, Debug)]
pub struct SshTunnel {
    pub meta: TunnelMeta,
    pub remote_host: String,
    pub remote_port: u16,
    pub user_name: String,
    pub identify_file: PathBuf,
    pub listen_host: String,
    pub listen_port: u16,
}

impl SshTunnel {
    #[inline]
    pub fn control_path(&self, context: &Context) -> PathBuf {
        PathBuf::from_iter([
            context.control_path_directory(),
            PathBuf::from({
                let tunnel_name = self.name();
                let Self { user_name, remote_host, remote_port, .. } = self;
                format!("{tunnel_name}_{user_name}@{remote_host}:{remote_port}.socket")
            }),
        ])
    }

    pub fn control_path_option(&self, context: &Context) -> String {
        format!("ControlPath={path}", path = self.control_path(context).to_string_lossy())
    }
}

impl Tunnel for SshTunnel {
    #[inline]
    fn meta(&self) -> &TunnelMeta { &self.meta }

    #[inline]
    fn tunnel_type(&self) -> TunnelType { TunnelType::Ssh }

    #[inline]
    fn restart(&self, context: &Context) -> Result<(), Error> {
        let _unused = self.stop(context);
        self.start(context)
    }

    #[inline]
    fn start(&self, context: &Context) -> Result<(), Error> {
        if self.is_running(context)? {
            return Ok(());
        }

        let _result = Command::new("ssh")
            .args([
                "-o",
                &self.control_path_option(context),
                "-o",
                "ControlMaster=auto",
                "-f",
                "-N",
                "-D",
                &format!("{}:{}", self.listen_host, self.listen_port),
                "-i",
                &context.apply_path(&self.identify_file).to_string_lossy(),
                "-l",
                &self.user_name,
                "-p",
                &format!("{}", self.remote_port),
                &self.remote_host,
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|_| error::SpawnSshCommandSnafu)?
            .wait()
            .with_context(|_| error::WaitForSshProcessSnafu)?;

        Ok(())
    }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> {
        let _result = Command::new("ssh")
            .args(["-O", "exit", "-o", &self.control_path_option(context), &self.remote_host])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|_| error::SpawnSshCommandSnafu)?
            .wait()
            .with_context(|_| error::WaitForSshProcessSnafu)?;

        Ok(())
    }

    #[inline]
    fn is_running(&self, context: &Context) -> Result<bool, Error> {
        let output = Command::new("ssh")
            .args(["-O", "check", "-o", &self.control_path_option(context), &self.remote_host])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|_| error::SpawnSshCommandSnafu)?
            .wait()
            .with_context(|_| error::WaitForSshProcessSnafu)?;

        Ok(output.success())
    }
}
