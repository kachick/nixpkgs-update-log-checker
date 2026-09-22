use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    #[arg(
        short = 'p',
        long = "packages",
        help = "List of package names to check",
        num_args = 1..,
        required = true
    )]
    packages: Vec<String>,
}

pub fn parse_cli_args() -> anyhow::Result<Vec<String>> {
    let cli = Cli::parse();
    Ok(cli.packages)
}
