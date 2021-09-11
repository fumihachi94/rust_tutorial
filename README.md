# Rust Tutorial


<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [Rust Tutorial](#rust-tutorial)
- [Rust setup](#rust-setup)
  - [MacOS](#macos)
    - [Rust Install & Update](#rust-install-update)
    - [Version確認](#version確認)
    - [rustc : Compiling and Running](#rustc-compiling-and-running)
    - [Cargo](#cargo)
    - [Creating a Project](#creating-a-project)
    - [Building and Running](#building-and-running)
    - [Variables](#variables)
    - [Uninstall](#uninstall)
  - [Ubuntu](#ubuntu)
    - [curlでインストールに失敗した場合](#curlでインストールに失敗した場合)
- [VScode](#vscode)
  - [Extentions](#extentions)

<!-- /code_chunk_output -->

# Rust setup

## MacOS

```
$ sw_vers 
ProductName:	macOS
ProductVersion:	11.2.3
BuildVersion:	20D91
```

### Rust Install & Update

```sh
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
※環境パスの設定を反映させるために一度ターミナルを再起動して下さい。
```

curl : ()

```sh
$ rustup updateq
```

インストールされるもの
- rustc  : コンパイラ
- cargo  : パッケージマネージャー&ビルドシステム
- rustup : インストーラ 

### Version確認

```sh
$ rustc --version
$ rustup --version
$ cargo --version
```

### rustc : Compiling and Running

```sh
$ rustc main.rs
$ ./main
```

### Cargo

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

### Variables

`let`で変数を定義できるが、基本的にimmutableの変数となる。mutableな変数とするときは、`let mut`とする必要がある。

```rs
let foo = 5; // immutable
let mut bar = 5; // mutable
```

### Uninstall

```sh
$ rustup self uninstall
```

---

## Ubuntu

### curlでインストールに失敗した場合

Ubuntu18.04 LTS環境(`VERSION="18.04.5 LTS (Bionic Beaver)"
`)でcurlインストールをしようとしたところ、以下のような証明書の検証で失敗しました。

```sh
$ curl https://sh.rustup.rs -sSf | sh
curl: (60) Cert verify failed: BADCERT_NOT_TRUSTED
More details here: https://curl.haxx.se/docs/sslcerts.html

curl failed to verify the legitimacy of the server and therefore could not
establish a secure connection to it. To learn more about this situation and
how to fix it, please visit the web page mentioned above.
```

**対処**

セキュアな通信が確認できないため上記のエラーが発生しているように思われるので、
スタンドアローンインストーラを利用して手動インストールします。
ただこの方法だとcargoが入らない…。

[Other Installation Methods - Rust Forge](https://forge.rust-lang.org/infra/other-installation-methods.html)

`x86_64-unknown-linux-gnu`のstable (1.54.0)	版`tar.gz`ファイルをダウンロードする。

```sh
$ tar -xzvf rust-1.54.0-x86_64-unknown-linux-gnu.tar.gz
$ cd rust-1.54.0-x86_64-unknown-linux-gnu
$ sudo sh install.sh 
[sudo] fumihachi のパスワード: 
install: creating uninstall script at /usr/local/lib/rustlib/uninstall.sh
install: installing component 'rustc'
install: installing component 'cargo'
install: installing component 'clippy-preview'
install: installing component 'rust-demangler-preview'
install: installing component 'rls-preview'
install: installing component 'rustfmt-preview'
install: installing component 'llvm-tools-preview'
install: installing component 'rust-analysis-x86_64-unknown-linux-gnu'
install: installing component 'rust-std-x86_64-unknown-linux-gnu'
install: installing component 'rust-docs'

    rust installed.
```

# lint

Rustコンパイラであるrustcには組込みのlintがあります。
以下のサイトが詳しく記載してくれています。

[Rustコンパイラ組込みのlintについて調べる - 低レイヤ強くなりたい組込み屋さんのブログ](https://tomo-wait-for-it-yuki.hatenablog.com/entry/2019/05/10/185418)

---

# VScode

開発環境としてVScodeを利用する場合のメモです。

## Extentions

- Rust : `rust-lang.rust` を入れておきましょう。