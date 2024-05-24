use std::path::PathBuf;

use asciidoc::telemetry::{get_subscriber, init_subscriber};
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
struct Cli {
    /// The file to parse.
    file: PathBuf,

    /// The format to output.
    #[arg(short, long, default_value_t = OutputFormat::Html)]
    format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    Html,
    Markdown,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            OutputFormat::Html => "html",
            OutputFormat::Markdown => "markdown",
        };
        write!(f, "{}", out)
    }
}

#[derive(Subcommand)]
enum Commands {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Begin by setting up tracing
    let subscriber = get_subscriber("asciidoc".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    start().await
}

#[tracing::instrument]
async fn start() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("Output format: {}", cli.format);

    println!("File to parse: {}", cli.file.display());

    Ok(())
}
