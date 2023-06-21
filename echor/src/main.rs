use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// A list of strings to display
    strings: Vec<String>,
    
    /// Do not print the trailing new line character
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    let string = args.strings.join(" ");

    if args.omit_newline {
        print!("{}", string);
    } else {
        println!("{}", string);
    }
}
