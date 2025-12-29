use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    // Import data from Dofus DB site and save locally
    #[arg(short = 'i', long = "import")]
    pub(crate) import: bool,

    // Parse local DofusDB json files into Rust models and export them
    #[arg(short = 'e', long = "export")]
    pub(crate) export: bool,
}