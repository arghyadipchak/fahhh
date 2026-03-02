use std::env;

use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}

const BASH_INIT: &str = include_str!("fahhh.bash");
const FISH_INIT: &str = include_str!("fahhh.fish");
const ZSH_INIT: &str = include_str!("fahhh.zsh");

impl Shell {
    pub fn gen_init(&self) -> String {
        let bin_path = match env::current_exe() {
            Ok(path) => path.to_string_lossy().to_string(),
            Err(_) => option_env!("CARGO_PKG_NAME").unwrap_or("fahhh").to_string(),
        };

        match self {
            Self::Bash => BASH_INIT,
            Self::Fish => FISH_INIT,
            Self::Zsh => ZSH_INIT,
        }
        .replace("::FAHHH::", &bin_path)
        .trim_end_matches('\n')
        .to_string()
    }
}
