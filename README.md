# nixpkgs-update-log-checker

Get a quick overview of whether the automatic update scripts for packages you care about are working or broken.

## Usage

### Basics

```plaintext
Usage: nixpkgs-update-log-checker --packages <packages>...

Options:
  -p, --packages <packages>...  List of package names to check
```

### From nix flake

```console
nix run github:kachick/nixpkgs-update-log-checker -- --packages pname
```

### Check your maintained packages

```bash
pnames="$(
  curl -H 'User-Agent: Mozilla' "https://repology.org/api/v1/projects/?search=&maintainer=$(git config user.email)&inrepo=nix_unstable" |
    jq --raw-output 'keys | join(" ")'
)"

echo "$pnames" | xargs nixpkgs-update-log-checker --packages
```

Repology might return different package names, [extracting from nixpkgs](https://discourse.nixos.org/t/how-to-get-a-list-of-packages-maintained-by-someone/29963/3) is another solution.

## Limitation

- The log analysis is based on experience, so it might give wrong results if it sees a pattern I’m not familiar with.
- Ideally we should respect [these skipped logs](https://github.com/nix-community/nixpkgs-update/blob/363f92cdbbf57bb13eec95c22c2b068d45fa2cea/src/Skiplist.hs#L168),
  however just handled as a failure

## Resources

- [List of update logs](https://nixpkgs-update-logs.nix-community.org/)
- [Upstream](https://github.com/nix-community/nixpkgs-update)

## Motivation

I wanted to keep track of the packages I maintain or depend on.\
Another reason is that I came across [this text](https://github.com/nix-community/nixpkgs-update/blob/363f92cdbbf57bb13eec95c22c2b068d45fa2cea/doc/details.md#L64-L67).

> There are a lot of packages `nixpkgs-update` currently has no hope of updating.\
> Please dredge the logs to find out why your pet package is not receiving updates.
