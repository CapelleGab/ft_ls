use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ft_ls", version, about = "List all files in directory like unix")]
struct Args {
    destination: String,
    // #[arg(short = 'v', long)]
    // verbose: bool
}

fn main() {
    let args = Args::parse();
}
