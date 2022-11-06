use color_eyre::eyre::{eyre, Result, WrapErr};
use digital_garden::write;
use directories::UserDirs;
use std::path::PathBuf;
use structopt::StructOpt;

/// A CLI for growing and curating a digital garden!
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    //parse the path string
    #[structopt(parse(from_os_str), short = "p", long, env)]
    garden_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Write something in your garden
    Write {
        /// Optionally set a title for what you are going to write about
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| {
        eyre!("Could not get home directory. Are you sure you are on a computer?")
    })?;
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    let garden_path = match opt.garden_path {
        //if the user passed in a path, use that
        Some(pathbuf) => Ok(pathbuf),
        //otherwise, use the default
        None => get_default_garden_dir().wrap_err("garden_path was not supplied"),
    }?;

    match opt.cmd {
        Command::Write { title } => write(garden_path, title),
    }
}
