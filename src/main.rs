#[macro_use]
extern crate lazy_static;
mod anki;
mod cli;
use std::collections::HashMap;

//mod anki_connector;
use anyhow::Result;
use cli::CommandLineArgs;
use simple_logger::SimpleLogger;
use structopt::StructOpt;
use config::Config;

const NOTE_KEY: &str = "note_seperator";
const QUESTION_KEY: &str = "question_seperator";
const CSS_KEY: &str = "css";
const CODE_TYPE_KEY: &str = "code_style";

lazy_static! {
    static ref SETTING: Config =  {
        let mut setting = config::Config::default();
        let mut setting_default = config::Config::default();
        let setting = setting.merge(config::File::with_name("setting")).unwrap_or(&mut setting_default);
        setting.to_owned()
    };
    static ref DEFAULT_NOTE_SEQ:String = SETTING.get(NOTE_KEY).unwrap_or(String::from("\r\n\r\n\r\n")).to_owned();
    static ref DEFAULT_QUESTION_SEQ:String = SETTING.get(QUESTION_KEY).unwrap_or(String::from("---")).to_owned();
    static ref DEFAULT_CSS:String = SETTING.get(CSS_KEY)
                                            .unwrap_or(
                                                String::from(".card { font-family: arial; font-size: 18px; background: #e1e1db; }\n 
pre code { display: block; overflow-x: auto; background: #f6f7f6; color: #3b2e2a; padding: 0.5em; border-radius:10px;}\n
code { background: #f6f7f6;padding:0.1em; border-radius:3px; }\n
strong { color: #e41f1f; }\n
img { border-radius: 10px; }\n
table {font-family: verdana,arial,sans-serif;font-size:11px;color:#333333;border-width: 1px;border-color: #666666;border-collapse: collapse;}
table th {border-width: 1px;padding: 8px;border-style: solid;border-color: #666666;}
table td {border-width: 1px;padding: 8px;border-style: solid;border-color: #666666;}")).to_owned();
    static ref DEFAULT_CODE_TYPE:String = SETTING.get(CODE_TYPE_KEY).unwrap_or(String::from("Solarized (dark)")).to_owned();
}

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

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
