use clap::Parser;
use cli::Cli;
use error::Error;
use gui::new_pdf_window;

mod constants;
mod cli;
mod error;
mod gui;
mod mode;
mod util;

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    new_pdf_window(args.file_path.as_deref(), args.password.as_deref())
}
