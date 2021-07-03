use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Anki notes generate",
    about = "A command line app written in Rust"
)]
pub struct CommandLineArgs {
    /// 指定markdown文件名.务虚写后缀,默认与另外一个同名
    // #[structopt(parse(from_os_str), short, long)]
    #[structopt(short, long)]
    pub markdown_file: Option<String>,
    /// 指定apkg文件名.无需写后缀.默认与另外一个同名
    #[structopt(short, long)]
    pub apkg_file: Option<String>,
}
