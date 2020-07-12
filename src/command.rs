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
    #[structopt(long = "config-file")]
    config_file: Option<PathBuf>,

    #[structopt(subcommand)]
    subcommand: SubCommand,
}

impl Command {
    #[inline]
    pub fn new() -> Command { Command::from_args() }

    #[inline]
    pub fn app_name() -> String { Command::clap().get_name().to_owned() }

    pub fn default_config_file() -> PathBuf {
        let mut p = dirs::config_dir().unwrap();
        p.push(Command::clap().get_name());
        p.push("default.yaml");
        p
    }

    pub fn run(self) -> Result<(), Error> {
        let (context, manager) = if self.subcommand.is_standalone() {
            (None, None)
        } else {
            let config_file = self.config_file.unwrap_or(Self::default_config_file());
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
    /// Shows current version
    Version,

    /// Shows shell completion
    Completions { shell: Shell },

    /// Shows available tunnels
    ListTunnels,

    #[structopt(alias = "up")]
    /// Starts a tunnel
    Start { tunnel: String },

    #[structopt(alias = "down")]
    /// Stops a tunnel
    Stop { tunnel: String },

    /// Restarts a tunnel
    Restart { tunnel: String },

    /// Check if tunnel is running
    Running { tunnel: String },

    /// Starts all available tunnels
    StartAll,

    /// Stops all available tunnels
    StopAll,

    /// Restarts all available tunnels
    RestartAll,
}

impl SubCommand {
    #[inline]
    pub fn is_standalone(&self) -> bool {
        match self {
            SubCommand::Version | SubCommand::Completions { .. } => true,
            _ => false,
        }
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
                    let description = tunnel.description.unwrap_or(String::new());
                    println!("{:24}\t{}", name, description);
                });
                Ok(())
            }
            (SubCommand::Start { tunnel }, Some(manager), Some(context)) => {
                manager.start(&context, &tunnel)
            }
            (SubCommand::Stop { tunnel }, Some(manager), Some(context)) => {
                manager.stop(&context, &tunnel)
            }
            (SubCommand::Restart { tunnel }, Some(manager), Some(context)) => {
                manager.restart(&context, &tunnel)
            }
            (SubCommand::Running { tunnel }, Some(manager), Some(context)) => {
                let is_running = manager.is_running(&context, &tunnel)?;
                println!("{}", is_running);
                Ok(())
            }
            (SubCommand::StartAll, Some(manager), Some(context)) => manager.start_all(&context),
            (SubCommand::StopAll, Some(manager), Some(context)) => manager.stop_all(&context),
            (SubCommand::RestartAll, Some(manager), Some(context)) => manager.restart_all(&context),
            (..) => Ok(()),
        }
    }
}
