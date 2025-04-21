use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;

mod cli;
mod log_analysis;
mod package_checker;

#[tokio::main]
async fn main() -> Result<()> {
    let packages = cli::parse_cli_args()?;
    let client = Client::builder().build()?;
    let results = join_all(
        packages
            .iter()
            .map(|pkg| package_checker::check_package(&client, pkg)),
    )
    .await;

    for (pkg, result) in packages.iter().zip(results) {
        match result {
            Ok(res) => println!("{}: {}", pkg, res),
            Err(_) => println!("\x1b[31m[ERROR]\x1b[0m {}: Unknown error to analyze", pkg),
        }
    }

    Ok(())
}
