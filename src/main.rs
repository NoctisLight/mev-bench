use clap::Parser;
mod providers;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    provider: Option<String>,
    #[arg(long)]
    test: Option<String>,
    #[arg(long, default_value="5")]
    rounds: usize,
    #[arg(long)]
    full: bool,
    #[arg(long)]
    json: bool,
 }



fn main() {
    let cli = Cli::parse();
    println!("Selected : {:?}", cli);
}
