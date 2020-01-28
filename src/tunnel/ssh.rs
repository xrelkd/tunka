use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::context::Context;
use crate::error::Error;
use crate::tunnel::{Tunnel, TunnelMeta, TunnelType};

#[derive(Debug, Clone)]
pub struct SshTunnel {
    meta: TunnelMeta,
    remote_host: String,
    remote_port: u16,
    user_name: String,
    identify_file: PathBuf,
    listen_host: String,
    listen_port: u16,
}

impl SshTunnel {
    pub fn new<P: AsRef<Path>>(
        name: &str,
        description: Option<String>,
        remote_host: &str,
        remote_port: u16,
        user_name: &str,
        identify_file: P,
        listen_host: &str,
        listen_port: u16,
    ) -> SshTunnel {
        let meta = TunnelMeta { name: name.to_owned(), description };
        SshTunnel {
            meta,
            remote_host: remote_host.to_owned(),
            remote_port,
            user_name: user_name.to_owned(),
            identify_file: identify_file.as_ref().into(),
            listen_host: listen_host.to_owned(),
            listen_port,
        }
    }

    #[inline]
    pub fn control_path(&self, context: &Context) -> PathBuf {
        let mut p = PathBuf::from(context.control_path_directory());
        p.push(format!(
            "{}_{}@{}:{}.socket",
            self.name(),
            self.user_name,
            self.remote_host,
            self.remote_port
        ));
        p
    }

    pub fn control_path_option(&self, context: &Context) -> String {
        format!("ControlPath={}", self.control_path(context).to_string_lossy())
    }
}

impl Tunnel for SshTunnel {
    #[inline]
    fn name(&self) -> &str {
        &self.meta.name
    }

    #[inline]
    fn meta(&self) -> &TunnelMeta {
        &self.meta
    }

    #[inline]
    fn tunnel_type(&self) -> TunnelType {
        TunnelType::Ssh
    }

    #[inline]
    fn restart(&self, context: &Context) -> Result<(), Error> {
        let _ = self.stop(context);
        self.start(context)
    }

    #[inline]
    fn start(&self, context: &Context) -> Result<(), Error> {
        if self.is_running(context)? {
            return Ok(());
        }

        Command::new("ssh")
            .args(&[
                "-o",
                &self.control_path_option(&context),
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
            .spawn()?
            .wait()?;

        Ok(())
    }

    #[inline]
    fn stop(&self, context: &Context) -> Result<(), Error> {
        Command::new("ssh")
            .args(&["-O", "exit", "-o", &self.control_path_option(&context), &self.remote_host])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(())
    }

    #[inline]
    fn is_running(&self, context: &Context) -> Result<bool, Error> {
        let output = Command::new("ssh")
            .args(&["-O", "check", "-o", &self.control_path_option(&context), &self.remote_host])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?
            .wait()?;

        Ok(output.success())
    }
}
