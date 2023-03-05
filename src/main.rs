use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Input city name for which you want to check the weather.
    #[arg(short, long)]
    city_name: String
}

fn main() {
    let args = Args::parse();
    println!("{:?}", &args.city_name);
}