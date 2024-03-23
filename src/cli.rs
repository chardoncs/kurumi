use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(help = "Path of file to open")]
    pub file_path: Option<String>,

    #[clap(short, long, help = "Password")]
    pub password: Option<String>,
}
