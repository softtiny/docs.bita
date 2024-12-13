use bitar::{
    archive_reader::IoReader, 
    Archive, 
    //CloneOutput,
};
//use futures_util::{StreamExt, TryStreamExt};
use tokio::fs::{
    File, 
    //OpenOptions,
};

// `main` function is not allowed to be `async`
//tokio = { version = "1", features = ["io-util"] }
// tokio = { version = "1.24.1", features = [
//   "fs",
//   "io-std",
//   "macros",
//   "time",
//   "rt-multi-thread",
// ] }

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    //let output_name = "sickan.jpg";
    let input_path = "bitar/examples/resources/example-archive.cba";
    //let example_seed = "bitar/examples/resources/example.seed";

    // open archive  which  source  we want to clone
    let _archive = Archive::try_init(IoReader::new(File::open(input_path).await?)).await?;
    Ok(())
}