use anyhow::Result;

mod cli;
mod log_analysis;
mod package_checker;

fn main() -> Result<()> {
    let packages = cli::parse_cli_args()?;
    let config = ureq::config::Config::default();
    let agent = ureq::Agent::new_with_config(config);

    let results = std::thread::scope(|s| {
        let handles: Vec<_> = packages
            .iter()
            .map(|pkg| s.spawn(|| package_checker::check_package(&agent, pkg)))
            .collect();

        handles
            .into_iter()
            .map(|h| h.join().expect("Thread panicked"))
            .collect::<Vec<_>>()
    });

    let mut has_unexpected_error = false;

    for (pkg, result) in packages.iter().zip(results.iter()) {
        match result {
            Ok(res) => println!("{pkg}: {res}"),
            Err(_) => {
                println!("\x1b[31m[ERROR]\x1b[0m {pkg}: Unknown error to analyze");
                has_unexpected_error = true;
            }
        }
    }

    if has_unexpected_error
        || results.iter().any(|result| {
            matches!(
                result,
                Ok(package_checker::PackageCheckResult::Failure { .. })
            )
        })
    {
        std::process::exit(1);
    }

    Ok(())
}
