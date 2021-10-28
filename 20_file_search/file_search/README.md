
## 参考サイト

こちらのサイトが一番いいかも

[Rust - katapedia](https://yutakatay.gitbook.io/katapedia/doc/rust)

###  その他      

[Rustでファイルの一覧を取得してみる – 1/0](https://note.katsumataryo.com/tech/2019/09/1452.html)

[Rustで変数の型を調べる方法 | 非IT企業に勤める中年サラリーマンのIT日記](http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/)

[PathBuf in std::path - Rust](https://doc.rust-lang.org/std/path/struct.PathBuf.html)

### Rust 命名規則

-[命名 - Rust APIガイドライン](https://sinkuu.github.io/api-guidelines/naming.html#%E5%A4%A7%E6%96%87%E5%AD%97%E5%B0%8F%E6%96%87%E5%AD%97%E3%81%AE%E4%BD%BF%E3%81%84%E5%88%86%E3%81%91%E3%81%8Crfc430%E3%81%AB%E5%BE%93%E3%81%A3%E3%81%A6%E3%81%84%E3%82%8B-c-case)

### `?`演算子　エラーの委譲

- [Rustのエラー処理 - やってみる](https://ytyaru.hatenablog.com/entry/2020/09/07/000000)

### unwrap()とは？

- [rust - Rustの"unwrap()"は何をするものですか？ - スタック・オーバーフロー](https://ja.stackoverflow.com/questions/1730/rust%E3%81%AEunwrap%E3%81%AF%E4%BD%95%E3%82%92%E3%81%99%E3%82%8B%E3%82%82%E3%81%AE%E3%81%A7%E3%81%99%E3%81%8B)

### ファイル操作（fs::metadata）

- [Rust - 構造体 std::fs::メタデータ - ファイルに関するメタデータ情報。 この構造は、 metadata または symlink_metadata 関数またはメソッドから返され、ファイルのアクセス許可 - 日本語](https://runebook.dev/ja/docs/rust/std/fs/struct.metadata)

- [MetadataExt in std::os::unix::fs - Rust](https://doc.rust-lang.org/std/os/unix/fs/trait.MetadataExt.html)

- [Rust自習（std::time::SystemTime） - やってみる](https://ytyaru.hatenablog.com/entry/2020/12/15/000000)


### chrono扱い方

- [chrono - crates.io: Rust Package Registry](https://crates.io/crates/chrono)

epoch to UTC date

> Use `Utc.timestamp(seconds, nanoseconds)` to construct a `DateTime<Utc>` from a UNIX timestamp (seconds, nanoseconds that passed since January 1st 1970).
> Use `DateTime.timestamp` to get the timestamp (in seconds) from a `DateTime`. Additionally, you can use `DateTime.timestamp_subsec_nanos` to get the number of additional number of nanoseconds.

```rust
// We need the trait in scope to use Utc::timestamp().
use chrono::{DateTime, TimeZone, Utc};

// Construct a datetime from epoch:
let dt = Utc.timestamp(1_500_000_000, 0);
assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

// Get epoch value from a datetime:
let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
assert_eq!(dt.timestamp(), 1_500_000_000);
```

型変換 u64->i64

```rust
let a:u64 = 1.0;
let b = a as i64
```

[キャスト](https://doc.rust-jp.rs/rust-nomicon-ja/casts.html)

### UNIX_EPOCH

- [UNIX_EPOCH in std::time - Rust](https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html)

### Rustからファイルを外部アプリで開く

[Command in std::process - Rust](https://doc.rust-lang.org/std/process/struct.Command.html)

```rust
use std::process::Command;
// Linux
let mut output = Command::new("xdg-open").arg("./xxxx.png").spawn();
```

### OSで条件分け：プラットフォーム依存処理

[Rustでプラットフォーム依存の処理を書く - ryochack.blog](https://ryochack.hatenablog.com/entry/2018/10/14/112957)

### Windows

[[Rust/WinRT] 既定のプログラムを使用してファイルを開く - Qiita](https://qiita.com/osanshouo/items/c83a25c3842d61b055b1)