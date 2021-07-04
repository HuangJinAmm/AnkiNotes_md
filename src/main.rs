mod anki;
mod cli;
//mod anki_connector;
use structopt::StructOpt;

use cli::CommandLineArgs;

fn main() {
    let CommandLineArgs {
        markdown_file,
        apkg_file,
    } = CommandLineArgs::from_args();

    match (markdown_file, apkg_file) {
        (Some(md), Some(apkg)) => {
            anki::generate_apkg(md, apkg).unwrap();
        }
        (Some(md), None) => {
            anki::generate_apkg(md.clone(), md).unwrap();
        }
        (None, Some(apkg)) => {
            anki::generate_apkg(apkg.clone(), apkg).unwrap();
        }
        (None, None) => {
            anki::generate_apkg_from_current_dir();
        }
    };
}
