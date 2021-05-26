use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub fn generate_apkg(filename: PathBuf) ->Result<()>{
    let contents = fs::read_to_string(filename).expect("无法打开文件");
    let temp = contents.replace("\r\n","\n");
    let lines = temp.split("\n");

    for line in lines{
        println!("{}",contents);
    }

    Ok(())
}

