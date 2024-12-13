use bitar::{archive_reader::IoReader, Archive, CloneOutPut};
use futures_util::{StreamExt, TryStreamExt};
use tokio::fs::{File, OpenOptions};

#[tokio::main]
async fn main() -> Result((),Box<dyn std::error::Error>) {
    let output_name = "sickan.jpg";
    let input_path = "bitar/examples/resources/example-archive.cba";
    let example_seed = "bitar/examples/resources/example.seed";

    // open archive  which  source  we want to clone
    let mut archive = Archive::try_init(IoReader::new(File::open(input_path).await?)).await?;
    Ok(())
}