# Rust Tutorial


<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [Rust Tutorial](#rust-tutorial)
  - [Environment](#environment)
  - [Rust Install & Update](#rust-install-update)
  - [Version確認](#version確認)
  - [Uninstall](#uninstall)
- [VScode](#vscode)
  - [Extentions](#extentions)

<!-- /code_chunk_output -->


## Environment

```
$ sw_vers 
ProductName:	macOS
ProductVersion:	11.2.3
BuildVersion:	20D91
```

## Rust Install & Update

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
※環境パスの設定を反映させるために一度ターミナルを再起動して下さい。
```

```sh
$ rustup updateq
```

インストールされるもの
- rustc  : コンパイラ
- cargo  : パッケージマネージャー&ビルドシステム
- rustup : インストーラ 

## Version確認

```sh
$ rustc --version
$ rustup --version
$ cargo --version
```

## rustc : Compiling and Running

```sh
$ rustc main.rs
$ ./main
```

## Cargo

Cargoがやってくれること
- コードのビルド
- 依存ライブラリのダウンロードとビルド
など、基本的な便利機能を備えている。

### Creating a Project

```sh
$ cargo new hello_cargo
$ cd hello_cargo
```
※プロジェクト名の先頭に数字は使えません（エラーになります）
※新規にGit管理する場合はは以下のように`--vsc`オプションを指定して下さい。

```
$ cargo new --vcs=git hello_cargo
```

### Building and Running

```sh
$ cargo build
$ ./target/debug/hello_cargo
Hello, world!
```

コンパイルと実行を同時に実施したい場合は`run`コマンドを使用できる。

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello_cargo`
Hello, world!
```

実行せずにコンパイルされているかチェックする場合は`check`コマンドを利用できます。

```sh
$ cargo check
    Checking hello_cargo v0.1.0 (/Users/fumi/Dev/wshub/rust_tutorial/02_hello_cargo/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
```

プロジェクトをリリースモードでbuildしたい場合は、`--release`オプションを指定できます。

```sh
$ cargo build --release     
   Compiling hello_cargo v0.1.0 (/Users/fumi/Dev/wshub/rust_tutorial/02_hello_cargo/hello_cargo)
    Finished release [optimized] target(s) in 0.25s
```


## Uninstall

```sh
$ rustup self uninstall
```

---

# VScode

開発環境としてVScodeを利用する場合のメモです。

## Extentions

- Rust : `rust-lang.rust` を入れておきましょう。