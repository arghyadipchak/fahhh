mod error;
mod init;

use std::{io::Cursor, process::ExitCode};

use clap::Parser;
use rodio::{Decoder, DeviceSinkBuilder, Player};

use crate::{error::FahhhError, init::Shell};

const SOUND: &[u8] = include_bytes!("../assets/fahhh.mp3");

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    #[command(about = "Print shell init code")]
    Init {
        #[arg()]
        shell: Shell,
    },
    #[command(about = "Play sound")]
    Play,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { shell } => {
            println!("{}", shell.gen_init());
            ExitCode::SUCCESS
        }
        Command::Play => match play() {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("{err}");
                ExitCode::FAILURE
            }
        },
    }
}

fn play() -> Result<(), FahhhError> {
    let mut handle = DeviceSinkBuilder::open_default_sink()?;
    handle.log_on_drop(false);

    let source = Decoder::new(Cursor::new(SOUND))?;
    let player = Player::connect_new(handle.mixer());

    player.append(source);
    player.sleep_until_end();

    Ok(())
}
