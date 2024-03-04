use clap::{Parser, Subcommand};
use log::debug;
use notify::{RecursiveMode, Result, Watcher};
use std::{future::IntoFuture, path::Path, sync::mpsc::{Receiver, Sender}};

mod config;
mod processor;
mod sink;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[clap(subcommand)]
    subcommands: SubCommands,
}
#[derive(Subcommand, Debug)]
enum SubCommands {
    Run {
        #[clap(short, long, required = true)]
        options_path: String,
    },
}

async fn run(options_path: String) -> Result<()> {
    let configuration = config::load_config(&options_path).unwrap();
    if configuration.watch_paths.len() == 0 {
        println!("No paths to watch");
        return Ok(());
    }
    // Build the processor
    let processor = processor::Processor::new(configuration.clone());

    // Setup a channel to send events
    let (tx,rx): (Sender<notify::Event>, Receiver<notify::Event>) = std::sync::mpsc::channel();
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
           tx.send(event).unwrap();
        }
    })?;

    for path in configuration.watch_paths {
        debug!("watching path: {}", path.path);

        let recursion = if path.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        watcher.watch(Path::new(&path.path), recursion)?;
    }
    loop {
        match rx.recv() {
            Ok(event) => {
                processor.process_event(event).await;
            }
            Err(e) => {
                eprintln!("watch error: {:?}", e);
            }
        }
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
