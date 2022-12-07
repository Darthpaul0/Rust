mod tasks;
mod cli;

use anyhow::{anyhow, Ok};
use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use tasks::Task;

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        // we use anyhow macro to produce a anyhow::Error
        // wich will give use a custom message
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
