use std::process::exit;

use crate::cli::Args;

mod cli;
mod errors;
mod proxy;
use proxy::proxy;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    match Args::parse() {
        Ok(args) => proxy(&args.client, &args.server).await,
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    }
}
