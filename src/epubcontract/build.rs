
use std::path::PathBuf;
use anyhow::{anyhow,Result,Error};

use capnpc::CompilerCommand;
const CAPNP_FILE: &str = "protocol.capnp";

fn prepare_capnp() -> Result<()> {
    CompilerCommand::new()
        .file(PathBuf::from("src").join(CAPNP_FILE))
        .output_path(PathBuf::from("."))
        .run().map_err(|e| anyhow!(format!("{:?}", e)))?;

    Ok(())
}

pub fn main() -> Result<()> {

    // Compile capnp protocol definition
    //prepare_capnp()?;


    Ok(())
}