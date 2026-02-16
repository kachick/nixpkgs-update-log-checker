# Nixpkgs Update State DB

`snapshots/yyyy-mm-dd-hh-mm/state.db` is an SQLite database.

## Useful Commands

### Inspect the Database

```bash
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db ".tables"
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db ".schema"
```

### Check the Update Queue

```bash
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db "SELECT * FROM queue LIMIT 10;"
```

### Identify Failed Packages (Exit Code based)

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db 
  "SELECT attr_path, datetime(started, 'unixepoch') as start_time, exit_code FROM log WHERE exit_code != 0 ORDER BY started DESC LIMIT 20;"
```

### Monitor Queue Progress

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db 
  "SELECT is_dequeued, count(*) FROM queue GROUP BY is_dequeued;"
```

### Check Latest Result for a Specific Package

The `log` table only keeps the latest result for each package. History is not preserved in a single DB file.

```bash
# Example: Check dprint
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db 
  "SELECT attr_path, datetime(started, 'unixepoch', 'localtime') as start_time, datetime(finished, 'unixepoch', 'localtime') as end_time, (finished - started) as duration_sec, exit_code FROM log WHERE attr_path = 'dprint';"
```

### Find Long-Running Tasks

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db 
  "SELECT attr_path, (finished - started) as duration_sec FROM log WHERE finished IS NOT NULL ORDER BY duration_sec DESC LIMIT 20;"
```

### Identify Likely Failing Packages (Persistence based)

Packages that have finished their latest run but still remain in the `queue` table are likely failing (e.g., build errors).

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db 
  "SELECT l.attr_path, datetime(l.finished, 'unixepoch', 'localtime') as last_finished, (SELECT count(*) FROM queue q WHERE q.attr_path = l.attr_path) as queue_count 
   FROM log l 
   WHERE last_finished > datetime('now', '-7 days') 
     AND l.attr_path IN (SELECT attr_path FROM queue) 
   ORDER BY l.finished DESC LIMIT 20;"
```

### How supervisor.py Manages the Database

Analysis of the upstream `supervisor.py` reveals the following:

- **Meaning of `exit_code`**: It stores the exit code of the update process. Since build failures are handled internally by the process, it usually returns `0` as long as it finishes normally.
- **Role of `queue`**: Entries are not usually deleted. It is used for throttling and prioritization based on `last_started`.
- **Success Detection**: The supervisor only tracks task execution. It does not store domain-specific results, such as whether a PR was successfully created.

## Related Public Resources

In addition to individual log files, the following directories provide useful data:

- **`https://nixpkgs-update-logs.nix-community.org/~fetchers/`**:
    - Contains detailed output from fetcher runs (e.g., `github.123456789.txt`).
    - Useful for bulk checking version updates before they are dequeued.
- **`https://nixpkgs-update-logs.nix-community.org/~supervisor/`**:
    - Contains `state.db` and supervisor's own logs (`YYYY-MM-DD.stdout.log`).
    - Useful for debugging low-level process crashes.

## Schema Details

### `log` table

- `attr_path`: Nixpkgs attribute path (e.g., `dprint`)
- `started`: Start time (Unix epoch)
- `finished`: Finish time (Unix epoch)
- `exit_code`: Process exit code (0 usually means the process finished, NOT necessarily a successful update)
    - **Note**: Most build failures still result in `exit_code = 0`. You must parse the actual log file to determine success.

### `queue` table

- `attr_path`: Package name
- `is_dequeued`: 0: Pending, 1: Currently processing or processed by a worker
- `last_started`: Last attempt time
- `payload`: Metadata about the update (often version strings and URLs)
