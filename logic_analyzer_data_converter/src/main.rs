use clap::Parser;
use kingst_data_converter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    name: String,
}

const FAR_FILE_NAME: &'static str = "far.pcm";
const NEAR_FILE_NAME: &'static str = "near.pcm";

fn main() {
    println!("lib version is {}", kingst_data_converter::get_library_version());

    let args = Args::parse();
    println!("{:?}", args);

    kingst_data_converter::generate_pcm_file(&args.name, &[FAR_FILE_NAME, NEAR_FILE_NAME]);
}
