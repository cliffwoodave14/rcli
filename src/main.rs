
/// rcli csv -i input.csv -o output.json --header -d ','

use std::path::Path;

use clap:: Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Convert CSV to JSON format")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, help = "Input CSV file", default_value = "input.csv", value_parser = validate_input)]
    input: String,

    #[arg(short, long, help = "Output JSON file", default_value = "output.json")]  // "output.json".into()
    output: String,

    #[arg(short, long = "Delimiter", default_value_t = ',')]
    delimiter: char,

    #[arg(long, help = "CSV has header or not", default_value_t = true)]
    header: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

fn validate_input(input: &str) -> Result<String, &'static str> {
    if !Path::new(input).exists() {
        panic!("Input file does not exist");
    } else {
        Ok(input.into())
    }
}