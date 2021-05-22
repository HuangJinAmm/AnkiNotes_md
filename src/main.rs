mod cli;
mod task;
use structopt::StructOpt;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use task::Task;
use std::path::PathBuf;

fn main() ->anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("failed to find journal file"))?;

    match action {
        Add { task } => task::add_task(journal_file, Task::new(task)),
        List => task::list_tasks(journal_file),
        Done { positon } => task::complete_task(journal_file, positon),
    }?;
    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

