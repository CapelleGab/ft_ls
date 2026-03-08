use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ft_ping", version, about = "Send ICMP ECHO_REQUEST to network hosts")]
struct Args {
    destination: String,
    // #[arg(short = 'v', long)]
    // verbose: bool
}

fn main() {
    let args = Args::parse();
}
