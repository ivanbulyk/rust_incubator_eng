use dotenv::dotenv;

use step_1_4::Config;

fn main() {
    println!("Already implemented you!\n");

    let args: Vec<_> = std::env::args().collect();

    dotenv().ok();

    println!(
        "The path to its configuration file: {:#?}\n",
        Config::build(&args).file_path
    );
}
