use std::{io::Write, iter::FromIterator, path::PathBuf};

use clap::{CommandFactory, Parser};

use crate::{
    config::Config,
    context::{Context, ContextBuilder},
    error::Error,
    tunnel::TunnelManager,
};

#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Cli {
    #[arg(long = "config-file", help = "Configuration file path")]
    config_file: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

impl Default for Cli {
    fn default() -> Self { Cli::parse() }
}

impl Cli {
    #[inline]
    pub fn app_name() -> String {
        let app = Self::command();
        app.get_name().to_string()
    }

    #[inline]
    pub fn default_config_file() -> Result<PathBuf, Error> {
        Ok(PathBuf::from_iter([
            dirs::config_dir().ok_or(Error::UserConfigDirectoryNotFound)?,
            PathBuf::from(Self::app_name()),
            PathBuf::from("default.yaml"),
        ]))
    }

    pub fn run(self) -> Result<(), Error> {
        let (context, manager) = if self.command.is_standalone() {
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

        self.command.run(context, manager)
    }
}

#[derive(Debug, Parser)]
pub enum Command {
    #[command(aliases = &["ls"], about = "Shows available tunnels")]
    ListTunnels,

    #[command(aliases = &["up", "run"], about = "Starts a tunnel")]
    Start { tunnels: Vec<String> },

    #[command(aliases = &["down"], about = "Stops a tunnel")]
    Stop { tunnels: Vec<String> },

    #[command(about = "Restarts a tunnel")]
    Restart { tunnels: Vec<String> },

    #[command(about = "Checks whether a tunnel is running")]
    Running { tunnels: Vec<String> },

    #[command(about = "Starts all available tunnels")]
    StartAll,

    #[command(about = "Stops all available tunnels")]
    StopAll,

    #[command(about = "Restarts all available tunnels")]
    RestartAll,

    #[command(about = "Shows current version")]
    Version,

    #[command(about = "Generates shell completion")]
    Completions { shell: clap_complete::Shell },
}

impl Command {
    #[inline]
    pub fn is_standalone(&self) -> bool {
        matches!(self, Command::Version | Command::Completions { .. })
    }

    pub fn run(
        self,
        context: Option<Context>,
        manager: Option<TunnelManager>,
    ) -> Result<(), Error> {
        match (self, manager, context) {
            (Command::Version, ..) => {
                let mut stdout = std::io::stdout();
                stdout
                    .write_all(Cli::command().render_long_version().as_bytes())
                    .expect("failed to write to stdout");
                Ok(())
            }
            (Command::Completions { shell }, ..) => {
                let mut app = Cli::command();
                clap_complete::generate(shell, &mut app, Cli::app_name(), &mut std::io::stdout());
                Ok(())
            }
            (Command::ListTunnels, Some(manager), _) => {
                manager.metadata_list().into_iter().for_each(|tunnel| {
                    let name = tunnel.name;
                    let description = tunnel.description.unwrap_or_default();
                    println!("{name:24}\t{description}");
                });
                Ok(())
            }
            (Command::Start { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.start(&context, tunnel)?;
                }
                Ok(())
            }
            (Command::Stop { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.stop(&context, tunnel)?;
                }
                Ok(())
            }
            (Command::Restart { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    manager.restart(&context, tunnel)?;
                }
                Ok(())
            }
            (Command::Running { tunnels }, Some(manager), Some(context)) => {
                for tunnel in &tunnels {
                    let is_running = manager.is_running(&context, &tunnel)?;
                    println!("{is_running}");
                }
                Ok(())
            }
            (Command::StartAll, Some(manager), Some(context)) => manager.start_all(&context),
            (Command::StopAll, Some(manager), Some(context)) => manager.stop_all(&context),
            (Command::RestartAll, Some(manager), Some(context)) => manager.restart_all(&context),
            _ => Ok(()),
        }
    }
}
