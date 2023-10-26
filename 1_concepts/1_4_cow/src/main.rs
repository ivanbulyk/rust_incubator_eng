use dotenv::dotenv;

use step_1_4::Config;

fn main() {
    println!("Already implemented you!\n");

    dotenv().ok();

    println!(
        "The path to its configuration file: {:#?}\n",
        Config::build().file_path
    );
}
