use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
pub struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}
