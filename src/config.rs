use anyhow::{Context, Result, bail};
use regex::Regex;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct RootConfig {
    #[serde(default)]
    ignore: Vec<RawIgnoreRule>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawIgnoreRule {
    url: Option<String>,
    pattern: Option<String>,
    packages: Option<Vec<String>>,
    #[allow(dead_code)]
    reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IgnoreRule {
    url: Option<String>,
    pattern: Option<Regex>,
    packages: Option<Vec<String>>,
}

impl IgnoreRule {
    pub fn matches(&self, pname: &str, log_url: &str, raw_log: &str) -> bool {
        if let Some(pkgs) = &self.packages
            && !pkgs.iter().any(|p| p == pname)
        {
            return false;
        }
        if let Some(url) = &self.url
            && log_url != url
        {
            return false;
        }
        if let Some(pattern) = &self.pattern
            && !pattern.is_match(raw_log)
        {
            return false;
        }
        true
    }
}

pub fn load_ignore_rules(path: &Path) -> Result<Vec<IgnoreRule>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    parse_ignore_rules(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))
}

pub fn parse_ignore_rules(toml_str: &str) -> Result<Vec<IgnoreRule>> {
    let root: RootConfig = toml::from_str(toml_str)?;
    root.ignore
        .into_iter()
        .enumerate()
        .map(|(idx, raw)| compile_rule(idx, raw))
        .collect()
}

fn compile_rule(idx: usize, raw: RawIgnoreRule) -> Result<IgnoreRule> {
    if let Some(url) = &raw.url
        && url.is_empty()
    {
        bail!("ignore rule #{idx}: 'url' must not be empty");
    }

    if let Some(pkgs) = &raw.packages
        && pkgs.is_empty()
    {
        bail!("ignore rule #{idx}: 'packages' must not be empty when specified");
    }

    let pattern = match raw.pattern {
        Some(pat) => {
            if pat.is_empty() {
                bail!("ignore rule #{idx}: 'pattern' must not be empty");
            }
            Some(
                Regex::new(&pat)
                    .with_context(|| format!("ignore rule #{idx}: invalid regex '{pat}'"))?,
            )
        }
        None => None,
    };

    if raw.url.is_none() && pattern.is_none() {
        bail!("ignore rule #{idx}: at least one of 'url' or 'pattern' is required");
    }

    Ok(IgnoreRule {
        url: raw.url,
        pattern,
        packages: raw.packages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ignore_rules() {
        let toml = r#"
[[ignore]]
pattern = "go: updates to go\\.mod needed|go .* is required"
reason = "Allow Go version delay"

[[ignore]]
url = "https://nixpkgs-update-logs.nix-community.org/fresh-editor/2026-11-10.log"
packages = ["fresh-editor"]
reason = "Fixed upstream"
"#;

        let rules = parse_ignore_rules(toml).unwrap();
        assert_eq!(rules.len(), 2);

        assert!(rules[0].matches("pinact", "https://example.com/a.log", "go 1.24 is required"));
        assert!(!rules[0].matches("pinact", "https://example.com/a.log", "other error"));

        assert!(rules[1].matches(
            "fresh-editor",
            "https://nixpkgs-update-logs.nix-community.org/fresh-editor/2026-11-10.log",
            "nix build failed."
        ));
        assert!(!rules[1].matches(
            "other-pkg",
            "https://nixpkgs-update-logs.nix-community.org/fresh-editor/2026-11-10.log",
            "nix build failed."
        ));
    }

    #[test]
    fn test_parse_empty_config() {
        let rules = parse_ignore_rules("").unwrap();
        assert!(rules.is_empty());
    }

    #[test]
    fn test_rejects_unknown_top_level_field() {
        let toml = r#"
[skip]
hello = { hydra = "Test skip" }
"#;
        assert!(parse_ignore_rules(toml).is_err());
    }

    #[test]
    fn test_rejects_unknown_field_in_rule() {
        let toml = r#"
[[ignore]]
url = "https://example.com/a.log"
unknown_key = "oops"
"#;
        assert!(parse_ignore_rules(toml).is_err());
    }

    #[test]
    fn test_rejects_rule_without_url_or_pattern() {
        let toml = r#"
[[ignore]]
packages = ["pinact"]
reason = "Missing condition"
"#;
        assert!(parse_ignore_rules(toml).is_err());
    }

    #[test]
    fn test_rejects_invalid_regex() {
        let toml = r#"
[[ignore]]
pattern = "[unclosed"
"#;
        assert!(parse_ignore_rules(toml).is_err());
    }

    #[test]
    fn test_rejects_empty_url_pattern_or_packages() {
        assert!(parse_ignore_rules("[[ignore]]\nurl = \"\"\n").is_err());
        assert!(parse_ignore_rules("[[ignore]]\npattern = \"\"\n").is_err());
        assert!(
            parse_ignore_rules("[[ignore]]\nurl = \"https://example.com/a.log\"\npackages = []\n")
                .is_err()
        );
    }
}
