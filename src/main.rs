mod parsers;

fn main() {
    let cli_args: CliArgs = argh::from_env();

    let value = match &cli_args.subcommand {
        Format::Free(c) => c.execute(),
        Format::Df(c) => c.execute(),
    };

    if cli_args.pretty {
        println!(
            "{}",
            serde_json::to_string_pretty(&value).expect("pretty-print json value")
        );
    } else {
        println!(
            "{}",
            serde_json::to_string(&value).expect("print json value")
        );
    }
}

/// Parse the output of various command-line programs and convert them to JSON.
#[derive(argh::FromArgs)]
struct CliArgs {
    /// pretty-print the JSON output
    #[argh(switch, short = 'p')]
    pretty: bool,

    #[argh(subcommand)]
    subcommand: Format,
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
enum Format {
    Free(parsers::free::FreeCommand),
    Df(parsers::df::DfCommand),
}
