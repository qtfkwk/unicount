use {
    anyhow::Result,
    clap::Parser,
    clap_cargo::style::CLAP_STYLING,
    unescaper::unescape,
    unicount_lib::{Counter, Kind},
};

#[derive(Parser)]
#[command(about, version, max_term_width = 80, styles = CLAP_STYLING)]
struct Cli {
    /// Kind
    #[arg(short, long, default_value = "english-upper")]
    kind: Kind,

    /// Start
    #[arg(short, long, default_value = "0")]
    start: usize,

    /// Take
    #[arg(short, long, default_value = "100")]
    take: usize,

    /// Separator
    #[arg(short = 'S', long, default_value = "\\n")]
    separator: String,

    /// Custom alphabet
    #[arg(short, long)]
    alphabet: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let separator = unescape(&cli.separator)?;

    let counter = if let Some(alphabet) = cli.alphabet {
        Counter::new(&alphabet, cli.start)
    } else {
        cli.kind.counter(cli.start)
    };

    let last = cli.start + cli.take - 1;
    for (i, c) in counter.take(cli.take).enumerate() {
        print!("{c}");
        if i < last {
            print!("{separator}");
        }
    }
    println!();

    Ok(())
}
