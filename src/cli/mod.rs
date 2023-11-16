pub mod args;

use {crate::*, args::Args, clap::Parser, cli_log::*};

pub fn run() -> Result<(), NgxLaserError> {
    let args = Args::parse();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    // let _ = thumb(args.conf);
    let config = parse_config(args.conf)?;
    debug!("Success confuration");
    conf::init(config);
    let _ = server::start(args.port);
    log_mem(Level::Info);
    Ok(())
}
