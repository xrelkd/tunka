use std::path::{Path, PathBuf};

use crate::error::Error;

pub struct ContextBuilder {
    control_path_directory: PathBuf,
}

impl ContextBuilder {
    pub fn new() -> ContextBuilder {
        use crate::command::Command;
        let control_path_directory = PathBuf::from(format!("/tmp/{}", Command::app_name()));
        ContextBuilder { control_path_directory }
    }

    pub fn control_path_directory<P: AsRef<Path>>(mut self, dir: P) -> ContextBuilder {
        self.control_path_directory = dir.as_ref().to_owned();
        self
    }

    pub fn build(self) -> Result<Context, Error> {
        let user_name = std::env::var("USER").map_err(|_| Error::UserNameNotFound)?;
        let home_dir = dirs::home_dir()
            .map(|h| h.to_string_lossy().into())
            .ok_or(Error::HomeDirectoryNotFound)?;

        let control_path_directory = self.control_path_directory;
        Ok(Context { user_name, home_dir, control_path_directory })
    }
}

#[derive(Debug)]
pub struct Context {
    user_name: String,
    home_dir: String,
    control_path_directory: PathBuf,
}

impl Context {
    pub fn apply(&self, s: &str) -> String {
        s.replace("$USER", &self.user_name).replace("$HOME", &self.home_dir)
    }

    pub fn apply_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        path.as_ref().iter().map(|p| self.apply(&p.to_string_lossy())).collect()
    }

    pub fn control_path_directory(&self) -> PathBuf {
        self.apply_path(&self.control_path_directory)
    }
}
