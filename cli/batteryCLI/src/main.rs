use structopt::StructOpt;
fn main() {
    let args = Cli::from_args();
    if args.command == "battery"
    {
        println!("e");
    }
}
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    command: String
}
