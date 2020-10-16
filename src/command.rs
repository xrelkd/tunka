use std::path::PathBuf;

use structopt::{clap::Shell, StructOpt};

use crate::{
    config::Config,
    context::{Context, ContextBuilder},
    error::Error,
    tunnel::TunnelManager,
};

#[derive(Debug, StructOpt)]
pub struct Command {
    #[structopt(long = "config-file", help = "Configuration file path")]
    config_file: Option<PathBuf>,

    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl Command {
    #[inline]
    pub fn new() -> Command { Command::from_args() }

    #[inline]
    pub fn app_name() -> String { Command::clap().get_name().to_owned() }

    #[inline]
    pub fn default_config_file() -> Result<PathBuf, Error> {
        let mut p = dirs::config_dir().ok_or(Error::UserConfigDirectoryNotFound)?;
        p.push(Command::clap().get_name());
        p.push("default.yaml");
        Ok(p)
    }

    pub fn run(self) -> Result<(), Error> {
        let (context, manager) = if self.subcommand.is_standalone() {
            (None, None)
        } else {
            let config_file = self.config_file.unwrap_or(Self::default_config_file()?);
            let config = Config::from_file(config_file)?;
            let context = ContextBuilder::new()
                .control_path_directory(config.control_path_directory())
                .build()?;
            let manager = config.into_manager();
            (Some(context), Some(manager))
        };

        self.subcommand.run(context, manager)
    }
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(aliases = &["ls"], about = "Shows available tunnels")]
    ListTunnels,

    #[structopt(aliases = &["up", "run"], about = "Starts a tunnel")]
    Start { tunnels: Vec<String> },

    #[structopt(aliases = &["down"], about = "Stops a tunnel")]
    Stop { tunnels: Vec<String> },

    #[structopt(about = "Restarts a tunnel")]
    Restart { tunnels: Vec<String> },

    #[structopt(about = "Checks whether a tunnel is running")]
    Running { tunnels: Vec<String> },

    #[structopt(about = "Starts all available tunnels")]
    StartAll,

    #[structopt(about = "Stops all available tunnels")]
    StopAll,

    #[structopt(about = "Restarts all available tunnels")]
    RestartAll,

    #[structopt(about = "Shows current version")]
    Version,

    #[structopt(about = "Generates shell completion")]
    Completions { shell: Shell },
}

impl SubCommand {
    #[inline]
    pub fn is_standalone(&self) -> bool {
        matches!(self, SubCommand::Version | SubCommand::Completions { .. })
    }

    pub fn run(
        self,
        context: Option<Context>,
        manager: Option<TunnelManager>,
    ) -> Result<(), Error> {
        match (self, manager, context) {
            (SubCommand::Version, ..) => {
                Command::clap()
                    .write_version(&mut std::io::stdout())
                    .expect("failed to print version");
                Ok(())
            }
            (SubCommand::Completions { shell }, ..) => {
                let app_name = Command::app_name();
                Command::clap().gen_completions_to(app_name, shell, &mut std::io::stdout());
                Ok(())
            }
            (SubCommand::ListTunnels, Some(manager), _) => {
                manager.metadata_list().into_iter().for_each(|tunnel| {
                    let name = tunnel.name;
                    let description = tunnel.description.unwrap_or_default();
                    println!("{:24}\t{}", name, description);
                });
                Ok(())
            }
            (SubCommand::Start { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.start(&context, tunnel)?;
                }
                Ok(())
            }
            (SubCommand::Stop { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.stop(&context, tunnel)?;
                }
                Ok(())
            }
            (SubCommand::Restart { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.restart(&context, tunnel)?;
                }
                Ok(())
            }
            (SubCommand::Running { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    let is_running = manager.is_running(&context, &tunnel)?;
                    println!("{}", is_running);
                }
                Ok(())
            }
            (SubCommand::StartAll, Some(manager), Some(context)) => manager.start_all(&context),
            (SubCommand::StopAll, Some(manager), Some(context)) => manager.stop_all(&context),
            (SubCommand::RestartAll, Some(manager), Some(context)) => manager.restart_all(&context),
            (..) => Ok(()),
        }
    }
}
