use rpf7::{Rpf, RpfError};
use simple_logger::SimpleLogger;
use std::io::BufReader;

fn main() -> Result<(), RpfError> {
    SimpleLogger::new().env().init().unwrap();
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file.rpf>", args[0]);
        return Ok(());
    }

    log::info!("rpf file: {:?}", args[1]);
    let f = std::fs::File::open(&args[1])?;

    // read archive
    let mut rpf = Rpf::from_reader(BufReader::new(f));
    rpf.read_header()?;

    Ok(())
}
