mod anki;
mod cli;
//mod anki_connector;
use structopt::StructOpt;
use anyhow::Result;
use cli::CommandLineArgs;

fn main() -> Result<()>{
    let CommandLineArgs {
        markdown_file,
        apkg_file,
    } = CommandLineArgs::from_args();

    match (markdown_file, apkg_file) {
        (Some(md), Some(apkg)) => {
            anki::generate_apkg(md, apkg)?;
        }
        (Some(md), None) => {
            anki::generate_apkg(md.clone(), md)?;
        }
        (None, Some(apkg)) => {
            anki::generate_apkg(apkg.clone(), apkg)?;
        }
        (None, None) => {
            anki::generate_apkg_from_current_dir()?;
        }
    };
    Ok(())
}
