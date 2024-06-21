use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "adny", version, about)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}


#[derive(Debug, Parser)]
enum  SubCommand {
    #[command(name = "csv", about = "show CSV transformation")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,
    // default_value 实现了 FromStr 的 trait
    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(short, long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts)
}
