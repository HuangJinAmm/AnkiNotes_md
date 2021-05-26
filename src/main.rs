mod cli;
mod task;
mod anki;
use structopt::StructOpt;

use anyhow::anyhow;
use cli::{CommandLineArgs};
use task::Task;
use std::path::PathBuf;

fn main() ->anyhow::Result<()> {
    let CommandLineArgs {
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .ok_or(anyhow!("failed to find journal file"))?;

    anki::generate_apkg(journal_file)?;
    Ok(())
}



