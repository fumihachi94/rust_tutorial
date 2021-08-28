[Programming a Guessing Game - The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


## standard input 

```rust
let mut a = String::new();
std::io::stdin().read_line(&mut a)
```

## Standard output

```rust
let x = 1;
let y = 3;
println!("x = {}, y = {}", x, y);
```

```sh
> x = 1, y = 3
```

## 依存ライブラリの指定

crateを利用するときは`Cargo.toml`の`[dependencies]`に記載します。

例）

- Cargo.toml

```toml
[dependencies]
rand = "0.8.3"
```

- src/main.rs

```rust
se rand::Rng;

// 1~100の数字をランダムに生成
fn main(){
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("The randam number is: {}", rand_number);
}
```

## 依存ライブラリのバージョン更新

一度buildされると、`Cargo.lock`ファイルに依存ライブラリのバージョン情報が保存され、以降のbuildは`Cargo.lock`に記憶されているバージョンで実施されます。依存ライブラリのバージョンを更新したい場合は、`Cargo.toml`でバージョン変更し`$ cargo update`コマンドを使用する必要があります。


```toml
# >Cargo.toml
[dependencies]
rand = "0.8.4"
```

```sh
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

## cargoドキュメントの生成

```sh
$ cargo doc --open
```

作成されたドキュメントは`target/doc`内に格納される。
ドキュメントの作成方法については[公式サイト](https://doc.rust-jp.rs/book/second-edition/ch14-02-publishing-to-crates-io.html)を参考にして下さい。

基本的には

- ヘッダードキュメントの場合

```rust
//! # ここにMarkdown形式で文書を記述。
//! 
//! ドキュメントの記載方法について：
//! - [Crates.ioにクレートを公開する - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch14-02-publishing-to-crates-io.html)
```

- 各関数の説明の場合

```rust
/// ここに関数の説明を記載
/// 
```

## match構文の使い方

`which case:`文と似たような動作をする`match`.
下記例では、`cmp`で`var_A`と`var_B`を比較して、その大小を比較した結果当てはまるケース(`Ordering`)の処理を実行してくれる。

```rust
use std::cmp::Ordering;

match var_A.cmp(&var_B) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

## shadowing 

他の型から数値に変換する際に利用する方法。`guess_str`と`guess`のように型によって別の変数を定義する必要がない。

```rust
let mut guess = String::new();
guess = "10"

let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

`.trim()` : 文字列内の空白、及び`¥n`, `¥r¥n`を削除する関数






