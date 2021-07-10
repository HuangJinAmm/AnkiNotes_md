use std::path::PathBuf;

use structopt::StructOpt;

/// 将md文件按照指定的规则转换未anki卡片包
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Anki notes generate",
    about = "A command line app written in Rust"
)]
pub struct CommandLineArgs {
    ///需要处理的文件，没有则默认当前文件夹下面所有以md结尾的文件
    ///生成同名的apkg文件
    #[structopt(name = "FILE", parse(from_os_str))]
    pub files: Vec<PathBuf>,
}
