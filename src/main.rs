use std::path::Path;
use notify::{Watcher, RecursiveMode, Result};
use clap:: { Parser, Subcommand};
use log::debug;

mod config;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    subcommands: SubCommands,
}
#[derive(Subcommand, Debug)]
enum SubCommands {
    Run{
        #[clap(short, long,required = true)]
        options_path: String,
    }
}

async fn run(options_path: String) -> Result<()> {

        // Automatically select the best implementation for your platform.
        let mut watcher = notify::recommended_watcher(|res| {
            match res {
               Ok(event) => debug!("event: {:?}", event),
               Err(e) => debug!("watch error: {:?}", e),
            }
        })?;
        
        let configuration = config::load_config(&options_path).unwrap();
        if configuration.watch_paths.len() == 0 {
            println!("No paths to watch");
            return Ok(());
        }
        for path in configuration.watch_paths {
            debug!("watching path: {}", path);
            watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;
        }
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args: Args = Args::parse();
    match args.subcommands {
        SubCommands::Run { options_path } => {
            debug!("options_path: {}", options_path);
            run(options_path).await
        }
    }
}