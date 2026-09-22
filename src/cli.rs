use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(
        short = 'p',
        long = "packages",
        help = "List of package names to check",
        num_args = 1..,
        required = true
    )]
    pub packages: Vec<String>,

    #[arg(
        long = "fail-on-warning",
        help = "Exit with failure if any warning is encountered"
    )]
    pub fail_on_warning: bool,
}

pub fn parse_cli_args() -> anyhow::Result<Cli> {
    Ok(Cli::parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cli_args_without_fail_on_warning() {
        let cli = Cli::try_parse_from(["nixpkgs-update-log-checker", "-p", "dprint"]).unwrap();
        assert_eq!(cli.packages, vec!["dprint"]);
        assert!(!cli.fail_on_warning);
    }

    #[test]
    fn test_parse_cli_args_with_fail_on_warning() {
        let cli = Cli::try_parse_from([
            "nixpkgs-update-log-checker",
            "-p",
            "dprint",
            "--fail-on-warning",
        ])
        .unwrap();
        assert_eq!(cli.packages, vec!["dprint"]);
        assert!(cli.fail_on_warning);
    }
}
