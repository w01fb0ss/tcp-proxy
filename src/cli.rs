use structopt::StructOpt;

use crate::errors::Errcode;

#[derive(Debug, StructOpt)]
#[structopt(name = "proxy", about = "A simple tcp proxy.")]
pub struct Args {
    /// The address of the eyeball that we will be proxying traffic for
    #[structopt(short, long)]
    pub client: String,

    /// The address of the origin that we will be proxying traffic for
    #[structopt(short, long)]
    pub server: String,
}

impl Args {
    pub fn parse() -> Result<Args, Errcode> {
        let args = Args::from_args();

        setup_log(log::LevelFilter::Info);

        Ok(args)
    }
}

pub fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
