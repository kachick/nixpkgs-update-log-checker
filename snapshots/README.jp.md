# Nixpkgs Update State DB

`snapshots/yyyy-mm-dd-hh-mm/state.db` は SQLite 形式のデータベースです。

## Useful Commands

### データベースの中身を確認する

```bash
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db ".tables"
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db ".schema"
```

### パッケージの更新キューを確認する

```bash
nix develop --command sqlite3 snapshots/2026-02-16-07-32/state.db "SELECT * FROM queue LIMIT 10;"
```

### 失敗したパッケージを特定する

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db "SELECT attr_path, datetime(started, 'unixepoch') as start_time, exit_code FROM log WHERE exit_code != 0 ORDER BY started DESC LIMIT 20;"
```

### キューの消化状況を確認する

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db "SELECT is_dequeued, count(*) FROM queue GROUP BY is_dequeued;"
```

### 特定のパッケージの最新実行結果を確認する

`log` テーブルには各パッケージの最新の実行結果のみが保持されます（履歴が必要な場合は過去の DB スナップショットを参照する必要があります）。

```bash
# 例: dprint の結果を確認
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db \
  "SELECT attr_path, datetime(started, 'unixepoch', 'localtime') as start_time, datetime(finished, 'unixepoch', 'localtime') as end_time, (finished - started) as duration_sec, exit_code FROM log WHERE attr_path = 'dprint';"
```

### 実行に時間がかかっているパッケージを特定する

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db \
  "SELECT attr_path, (finished - started) as duration_sec FROM log WHERE finished IS NOT NULL ORDER BY duration_sec DESC LIMIT 20;"
```

### 失敗（または未完了）の可能性が高いパッケージを抽出する

`log` に記録（試行済み）があるにもかかわらず、依然として `queue` から削除されていないパッケージは、ビルドエラーなどで更新が完了しなかった可能性が高いです。

```bash
nix develop --command sqlite3 -header -column snapshots/2026-02-16-07-32/state.db \
  "SELECT l.attr_path, datetime(l.finished, 'unixepoch', 'localtime') as last_finished, (SELECT count(*) FROM queue q WHERE q.attr_path = l.attr_path) as queue_count \
   FROM log l \
   WHERE last_finished > datetime('now', '-7 days') \
     AND l.attr_path IN (SELECT attr_path FROM queue) \
   ORDER BY l.finished DESC LIMIT 20;"
```

### supervisor.py によるデータベース管理の仕組み

upstream の `supervisor.py` の解析により、以下の仕様が判明しました。

- **`exit_code` の意味**: パッケージ更新プロセスの終了コードがそのまま格納されます。ビルド失敗などの詳細はプロセス内部（ログ）で処理されるため、正常終了すれば成否に関わらず `0` になります。
- **`queue` の役割**: レコードは削除されず、スロットリングと優先順位制御（`last_started` が古いものから順に実行）に使用されます。
- **成否の判定**: supervisor は「成否」を管理しておらず、あくまで「タスクの実行」のみを管理しています。そのため、PRが作成されたか等のドメイン知識は DB には一切残りません。

## Schema Details

### `log` table

- `attr_path`: Nixpkgs の属性パス（例: `dprint`）
- `started`: 開始時刻（Unixタイムスタンプ）
- `finished`: 終了時刻（Unixタイムスタンプ）
- `exit_code`: 終了コード（0: 成功, その他: 失敗）
    - **注意**: 更新プロセスが完走した場合、ビルドに失敗しても `0` になることがほとんどです。パッケージの更新が実際に成功したか（PRが作成されたか等）を判定するには、別途ログファイルを解析する必要があります。
    - **比較例**: 失敗している `shogihome` (78s) と成功している `dprint` (97s) で、実行時間や終了コードに有意な差は見られませんでした。

### 関連する公開リソース

ログファイル以外にも、以下のディレクトリに有用な情報が配置されています。

- **`https://nixpkgs-update-logs.nix-community.org/~fetchers/`**:
    - 各 fetcher (github, repology 等) が更新を検知した際の詳細な出力（`github.123456789.txt` など）が置かれています。
    - DB の `queue` に入る前の「どのバージョンからどのバージョンへ上げようとしているか」という情報を、一括して取得するのに適しています。
- **`https://nixpkgs-update-logs.nix-community.org/~supervisor/`**:
    - `state.db` 自体のほか、supervisor の稼働ログ（`YYYY-MM-DD.stdout.log`）が置かれています。
    - プロセスがクラッシュした場合や、DB に反映されない低レイヤーのトラブルを確認するのに役立ちます。

### システム構成（nixpkgs-update.nix）からの知見

upstream の Nix 構成ファイルによると：
- 全てのデータは `/var/log/nixpkgs-update/` 配下に保存され、Nginx で公開されています。
- `/var/log/nixpkgs-update/~fetchers_history` には過去の fetcher 実行結果が蓄積されています（Webからも閲覧可能）。

### `queue` table

- `attr_path`: パッケージ名
- `is_dequeued`: 0: 未処理, 1: 処理済み
- `last_started`: 最後に開始された時刻
- `payload`: 更新に関するメタデータ（内部的な値の可能性あり）
