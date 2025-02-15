use clap::Parser;

#[derive(Parser)]
struct Args {
    branch_name: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let branch_name = args.branch_name.join("_").to_lowercase();

    println!("{}", branch_name);
}