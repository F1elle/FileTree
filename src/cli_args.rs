use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct CliArgs {
    /// Path to the directory
    path: Option<String>,
    /// Show time elapsed
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    time: bool
}

impl CliArgs {
    pub fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    pub fn get_time(&self) -> bool {
        self.time
    }
}

