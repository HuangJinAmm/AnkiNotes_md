mod anki;
mod cli;
//mod anki_connector;
use anyhow::Result;
use cli::CommandLineArgs;
use structopt::StructOpt;

fn main() -> Result<()> {
    let CommandLineArgs { files } = CommandLineArgs::from_args();

    if files.len() != 0 {
        for file in files {
            anki::generate_apkg(file)?;
        }
    } else {
        anki::generate_apkg_from_current_dir()?;
    }
    Ok(())
}
