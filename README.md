# nixpkgs-update-log-checker

[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fkachick%2Fnixpkgs-update-log-checker)](https://garnix.io/repo/kachick/nixpkgs-update-log-checker)

Tracker for automatic updates in nixpkgs

## Usage

### Basics

```plaintext
Usage: nixpkgs-update-log-checker --packages <packages>...

Options:
  -p, --packages <packages>...  List of package names to check
```

### Flake

```console
nix run github:kachick/nixpkgs-update-log-checker -- --packages pname
```

### Your maintained packages

```bash
pnames="$(NIX_PATH=nixpkgs=channel:nixpkgs-unstable nix run github:kachick/nixpkgs-maintained-by -- -id kachick)" 
echo "$pnames" | xargs nix run github:kachick/nixpkgs-update-log-checker -- --packages
```

See also [nixpkgs-maintained-by](https://github.com/kachick/nixpkgs-maintained-by).

## GitHub Actions

For example: [nixpkgs-health-check-action](https://github.com/kachick/nixpkgs-health-check-action)

## Limitation

- The log analysis is based on experience, so it might give wrong results if it sees a pattern I’m not familiar with.
- Ideally we should respect [these skipped logs](https://github.com/nix-community/nixpkgs-update/blob/363f92cdbbf57bb13eec95c22c2b068d45fa2cea/src/Skiplist.hs#L168),
  however supporting the small subset of it

## Resources

- [List of update logs](https://nixpkgs-update-logs.nix-community.org/)
- [Upstream](https://github.com/nix-community/nixpkgs-update)
- [Notifier](https://github.com/nix-community/nixpkgs-update/issues/476)

## Motivation

I wanted to keep track of the packages I maintain or depend on.\
Another reason is that I came across [this text](https://github.com/nix-community/nixpkgs-update/blob/363f92cdbbf57bb13eec95c22c2b068d45fa2cea/doc/details.md?plain=1#L64-L67).

> There are a lot of packages `nixpkgs-update` currently has no hope of updating.\
> Please dredge the logs to find out why your pet package is not receiving updates.
