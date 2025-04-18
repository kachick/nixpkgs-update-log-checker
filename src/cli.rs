use anyhow::Result;
use clap::{Arg, Command};

pub fn parse_cli_args() -> Result<Vec<String>> {
    let matches = Command::new("nixpkgs-update-log-checker")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("packages")
                .long("packages")
                .short('p')
                .help("List of package names to check")
                .num_args(1..)
                .required(true),
        )
        .get_matches();

    Ok(matches
        .get_many::<String>("packages")
        .unwrap()
        .map(|s| s.to_string())
        .collect())
}
