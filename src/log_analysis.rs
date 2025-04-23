use anyhow::Result;

pub enum LogAnalysisResult {
    Success { pr_url: Option<String> },
    Failure,
}

pub fn analyze_log(raw: &str) -> Result<LogAnalysisResult> {
    let lines: Vec<&str> = raw.lines().collect();

    if let Some(last_line) = lines.last() {
        let pr_prefix_for_api = "https://api.github.com/repos/NixOS/nixpkgs/pulls/";
        if let Some(pr_number) = last_line.strip_prefix(pr_prefix_for_api) {
            if pr_number.chars().all(|c| c.is_ascii_digit()) {
                let pr_url = format!("https://github.com/NixOS/nixpkgs/pull/{}", pr_number);
                return Ok(LogAnalysisResult::Success {
                    pr_url: Some(pr_url),
                });
            }
        }
    }

    let keywords = vec![
        "Packages updated!",
        "There might already be an open PR for this update:", // https://nixpkgs-update-logs.nix-community.org/tlrc/2025-03-17.log
        "An auto update branch exists with an equal or greater version", // https://nixpkgs-update-logs.nix-community.org/podman/2025-04-22.log,
        "No auto update branch exists", // https://nixpkgs-update-logs.nix-community.org/treefmt/2025-04-11.log
        "Do not update GNOME during a release cycle", // https://nixpkgs-update-logs.nix-community.org/loupe/2025-04-15.log
    ];
    if keywords.iter().any(|&keyword| raw.contains(keyword)) {
        return Ok(LogAnalysisResult::Success { pr_url: None });
    }

    Ok(LogAnalysisResult::Failure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_log_success_with_pr() {
        // Extracted from https://nixpkgs-update-logs.nix-community.org/dprint/2025-03-24.log
        let log = r#"- dprint-0.49.0: UPDATING ...
 - dprint-0.49.0: DONE.

Packages updated!

Diff after rewrites:

   pname = "dprint";
-  version = "0.49.0";
+  version = "0.49.1";

-    hash = "sha256-IhxtHOf4IY95B7UQPSOyLj4LqvcD2I9RxEu8B+OjtCE=";
+    hash = "sha256-6ye9FqOGW40TqoDREQm6pZAQaSuO2o9SY5RSfpmwKV4=";

-  cargoHash = "sha256-OdtUzlvbezeNk06AB6mzR3Rybh08asJJ3roNX0WOg54=";
+  cargoHash

[pull requests you find important]: https://github.com/NixOS/nixpkgs/pulls?q=is%3Aopen+sort%3Areactions-%2B1-desc
https://api.github.com/repos/NixOS/nixpkgs/pulls/392589"#;
        let result = analyze_log(log).unwrap();
        match result {
            LogAnalysisResult::Success { pr_url } => {
                assert_eq!(
                    pr_url,
                    Some("https://github.com/NixOS/nixpkgs/pull/392589".to_string())
                );
            }
            _ => panic!("Expected success with PR URL"),
        }
    }

    #[test]
    fn test_analyze_log_success_with_pr_for_font() {
        // Extracted from https://nixpkgs-update-logs.nix-community.org/plemoljp/2025-04-02.log
        // Which does not have "Packages updated!" in the log
        let log = r#"plemoljp 2.0.3 -> 2.0.4 https://github.com/yuru7/PlemolJP/releases

[pull requests you find important]: https://github.com/NixOS/nixpkgs/pulls?q=is%3Aopen+sort%3Areactions-%2B1-desc
https://api.github.com/repos/NixOS/nixpkgs/pulls/395562"#;
        let result = analyze_log(log).unwrap();
        match result {
            LogAnalysisResult::Success { pr_url } => {
                assert_eq!(
                    pr_url,
                    Some("https://github.com/NixOS/nixpkgs/pull/395562".to_string())
                );
            }
            _ => panic!("Expected success with PR URL"),
        }
    }

    #[test]
    fn test_analyze_log_success_but_no_pr() {
        // Extracted from https://nixpkgs-update-logs.nix-community.org/dprint/2025-04-13.log
        let log = "dprint 0 -> 1
attrpath: dprint
Checking auto update branch...

Press Enter key to continue...
Running update for:
Enqueuing group of 1 packages
 - dprint-0.49.1: UPDATING ...
 - dprint-0.49.1: DONE.

Packages updated!

The diff was empty after rewrites.";
        let result = analyze_log(log).unwrap();
        match result {
            LogAnalysisResult::Success { pr_url } => {
                assert_eq!(pr_url, None);
            }
            _ => panic!("Expected success and no PR URL"),
        }
    }

    #[test]
    fn test_analyze_log_failure() {
        // Extracted from https://nixpkgs-update-logs.nix-community.org/fishnet/2025-04-10.log
        let log = "fishnet 2.9.4 -> 2.9.5 https://github.com/lichess-org/fishnet/releases";
        let result = analyze_log(log).unwrap();
        assert!(matches!(result, LogAnalysisResult::Failure));
    }
}
