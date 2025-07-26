use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

use crate::log_analysis;

#[derive(Debug)]
pub enum PackageCheckResult {
    Success {
        log_url: String,
        pr_url: Option<String>,
    },
    Failure {
        log_url: String,
    },
    LogNotFound {
        log_list_url: String,
    },
    Skip {
        log_url: String,
    },
}

impl std::fmt::Display for PackageCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageCheckResult::Success { log_url, pr_url } => {
                if let Some(pr_url) = pr_url {
                    write!(f, "[UPDATED]: log={log_url}, pr={pr_url}")
                } else {
                    write!(f, "[AS-IS]: log={log_url}")
                }
            }
            PackageCheckResult::Failure { log_url } => {
                write!(f, "\x1b[31m[FAILURE]\x1b[0m: log={log_url}")
            }
            PackageCheckResult::LogNotFound { log_list_url } => {
                write!(
                    f,
                    "\x1b[33m[WARN]\x1b[0m: No logs found at {log_list_url}"
                )
            }
            PackageCheckResult::Skip { log_url } => {
                write!(f, "\x1b[33m[WARN]\x1b[0m: Skipped log={log_url}")
            }
        }
    }
}

// Returns all log URLs for a package, and sorted with LIFO order.
fn get_log_urls(raw_log_urls: &str, list_url: &Url) -> Result<Vec<String>> {
    let html = Html::parse_document(raw_log_urls);
    let anchor =
        Selector::parse("a").map_err(|e| anyhow::anyhow!("Failed to parse selector 'a': {}", e))?;

    let hrefs = html.select(&anchor).filter_map(|a| a.value().attr("href"));

    let log_hrefs = hrefs.filter(|href| href.ends_with(".log"));

    let mut log_urls: Vec<_> = log_hrefs
        .filter_map(|href| list_url.join(href).ok())
        .map(|url| url.to_string())
        .collect();

    log_urls.sort_by(|a, b| b.cmp(a));
    Ok(log_urls)
}

pub async fn check_package(client: &Client, pname: &str) -> Result<PackageCheckResult> {
    let log_list_url = Url::parse(&format!(
        "https://nixpkgs-update-logs.nix-community.org/{pname}/"
    ))
    .map_err(|e| anyhow::anyhow!("Failed to parse log list URL: {}", e))?;

    let raw_log_urls = client
        .get(log_list_url.as_str())
        .send()
        .await?
        .text()
        .await?;

    let log_urls = get_log_urls(&raw_log_urls, &log_list_url)
        .map_err(|e| anyhow::anyhow!("Failed to fetch logs: {}", e))?;

    if log_urls.is_empty() {
        return Ok(PackageCheckResult::LogNotFound {
            log_list_url: log_list_url.to_string(),
        });
    }

    let latest_log_url = log_urls[0].clone();
    let latest_log = client.get(&latest_log_url).send().await?.text().await?;
    match log_analysis::analyze_log(&latest_log)? {
        log_analysis::LogAnalysisResult::Success { pr_url } => Ok(PackageCheckResult::Success {
            log_url: latest_log_url,
            pr_url,
        }),
        log_analysis::LogAnalysisResult::Failure => Ok(PackageCheckResult::Failure {
            log_url: latest_log_url,
        }),
        log_analysis::LogAnalysisResult::NoUpdater => Ok(PackageCheckResult::Skip {
            log_url: latest_log_url,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_get_log_urls() {
        let list_url = Url::parse("https://nixpkgs-update-logs.nix-community.org/dprint/").unwrap();

        // Extracted from view-source:https://nixpkgs-update-logs.nix-community.org/dprint/
        let raw_log_urls = r#"
        <html>
        <head><title>Index of /dprint/</title></head>
        <body>
        <h1>Index of /dprint/</h1><hr><pre><a href="../">../</a>
            <a href="2023-10-30.log">2023-10-30.log</a>                                     30-Oct-2023 01:08                4800
            <a href="2023-11-21.log">2023-11-21.log</a>                                     22-Nov-2023 00:29               10200
            <a href="2023-12-15.log">2023-12-15.log</a>                                     15-Dec-2023 07:06                4800
            <a href="2023-12-26.log">2023-12-26.log</a>                                     26-Dec-2023 06:15                4837
            <a href="2024-01-01.log">2024-01-01.log</a>                                     01-Jan-2024 00:35                4837
            <a href="2024-01-08.log">2024-01-08.log</a>                                     08-Jan-2024 07:24                1340
            <a href="2024-04-28.log">2024-04-28.log</a>                                     28-Apr-2024 00:24                4813
            <a href="2024-05-30.log">2024-05-30.log</a>                                     30-May-2024 03:55                4813
            <a href="2024-06-11.log">2024-06-11.log</a>                                     11-Jun-2024 08:30                4813
            <a href="2024-06-19.log">2024-06-19.log</a>                                     19-Jun-2024 07:56                4902
            <a href="2024-07-01.log">2024-07-01.log</a>                                     01-Jul-2024 13:24                7090
            <a href="2024-07-08.log">2024-07-08.log</a>                                     08-Jul-2024 11:30                1340
            <a href="2024-07-21.log">2024-07-21.log</a>                                     21-Jul-2024 04:29                4683
            <a href="2024-10-20.log">2024-10-20.log</a>                                     20-Oct-2024 03:56                 146
            <a href="2024-12-06.log">2024-12-06.log</a>                                     06-Dec-2024 10:18                 891
            <a href="2024-12-17.log">2024-12-17.log</a>                                     17-Dec-2024 17:18                 836
            <a href="2024-12-28.log">2024-12-28.log</a>                                     28-Dec-2024 03:49                5708
            <a href="2025-01-07.log">2025-01-07.log</a>                                     07-Jan-2025 04:06                 836
            <a href="2025-01-19.log">2025-01-19.log</a>                                     19-Jan-2025 10:53                 836
            <a href="2025-02-01.log">2025-02-01.log</a>                                     01-Feb-2025 02:14                 836
            <a href="2025-02-15.log">2025-02-15.log</a>                                     15-Feb-2025 06:37                3734
            <a href="2025-02-28.log">2025-02-28.log</a>                                     28-Feb-2025 11:29                 836
            <a href="2025-03-12.log">2025-03-12.log</a>                                     12-Mar-2025 19:58                 866
            <a href="2025-03-24.log">2025-03-24.log</a>                                     24-Mar-2025 02:04                5754
            <a href="2025-04-04.log">2025-04-04.log</a>                                     04-Apr-2025 04:52                 866
            <a href="2025-04-13.log">2025-04-13.log</a>                                     13-Apr-2025 11:19                 866
        </pre><hr></body>
        </html>
        "#;

        let result = get_log_urls(raw_log_urls, &list_url).unwrap();
        let sample = result[..=1].to_vec();
        let expected = vec![
            "https://nixpkgs-update-logs.nix-community.org/dprint/2025-04-13.log".to_string(),
            "https://nixpkgs-update-logs.nix-community.org/dprint/2025-04-04.log".to_string(),
        ];

        assert_eq!(sample, expected);
    }

    #[test]
    fn test_get_log_urls_empty() {
        let raw_log_urls = r#"<html><body></body></html>"#;
        let list_url = Url::parse("https://example.com/logs/").unwrap();
        let result = get_log_urls(raw_log_urls, &list_url).unwrap();
        assert!(result.is_empty());
    }
}
