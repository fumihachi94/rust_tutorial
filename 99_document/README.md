[Introduction - Rust By Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/index.html)


<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [Ownership](#ownership)
  - [Ownershipのルール](#ownershipのルール)
  - [移動とコピーと参照と借用と](#移動とコピーと参照と借用と)
    - [移動 / Move](#移動-move)
    - [コピー / Copy](#コピー-copy)
    - [参照 / Reference](#参照-reference)
    - [借用 / Borrowing](#借用-borrowing)
- [基本データ型](#基本データ型)
  - [Scalar Types](#scalar-types)
  - [Compound Types](#compound-types)
    - [タプル / Tuple](#タプル-tuple)
    - [配列 / Array](#配列-array)
- [関数 / Function](#関数-function)
- [for文](#for文)
- [Errorの扱い](#errorの扱い)

<!-- /code_chunk_output -->

# Ownership

Rust最大の特徴がこの[ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)というものらしい。

ownershipシステムがメモリの割当/開放を管理してくれるため、設計者はメモリ管理に気を使うことが軽減される。
コンパイル時にコンパイラーがチェックする。

## Ownershipのルール

- Rust上の各値は`owner`と呼ばれる変数が割当てられる
- 各値に対し同時にひとつの`owner`しか存在できない
- `owner`がスコープから外れたら、メモリを自動的に開放してくれる
　（C++でも関数抜けるときに`delete`しなくてもいいようになっているのと似たようなものと思ってる）

## 移動とコピーと参照と借用と

### 移動 / Move

```rust
// Errorになるケース
// s1のownerがs2に移動(move)するため以降でs1は参照できなくなる
let s1 = String::from("Hello");
let s2 = s1;
println!("{}", s1);
```
### コピー / Copy

```rust
// OKのケース
// s1のownerがs2にコピー(Copy)されるため以降でもs1を参照できる
let s1 = String::from("Hello");
let s2 = s1.clone();
println!("{}", s1);
```

スタックに格納される（コンパイル時にサイズがわかっている）場合は、`clone()`を利用しなくてもコピーされます。
※上記の例ではヒープに格納されています。

```rust
// OKのケース
// s1がスタックに格納されるため、s2にコピー(copy)され以降でもs1を参照できる
let s1 = "Hello";
let s2 = s1;
println!("{}", s1);
```

### 参照 / Reference

- `immutable`の参照

ポインタを渡すがimmutableなので変更はできない。

```rust
// ポインタ渡し
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); <-- immutableなので変更できない
    s.len()
}
```

### 借用 / Borrowing

- `mutable`の参照

ポインタを渡し、mutableなので変更も可能。

```rust
fn main(){
    // ポインタ渡し
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world"); // <--変更可能
    s.len()
}
```










# 基本データ型

## Scalar Types

| Rust  | C/C++              | note                      |
| ----- | ------------------ | ------------------------- |
| i8    | signed char        |                           |
| i16   | short              |                           |
| i32   | int                |                           |
| i64   | long long          |                           |
| i128  | -                  |                           |
| isize | -                  | CPUアーキに合った型を取得 i32 or i64 |
| u8    | unsigned char      |                           |
| u16   | unsigned short     |                           |
| u32   | unsigned int       |                           |
| u64   | unsigned long long |                           |
| u128  | -                  |                           |
| usize | -                  | CPUアーキに合った型を取得 u32 or u64 |
| f32   | float              |                           |
| f64   | double             |                           |
| char  | char               |                           |
| bool  | bool               |                           |
| ()    | -                  | ユニット型: 要素数0のタプル型          |


参考サイト
- [[Rust] isize、usizeとは - Qiita](https://qiita.com/osorezugoing/items/23940e2507ae6149f12d)

## Compound Types

| Type | command | e.g.| Note |
| -- | -- | -- | -- |
| タプル | ( ) |(1, 2.0, true)| 各要素は異なる型を指定可能 | 
| 配列 | [ ] |[1, 1, 2, 3, 5]| 各要素は同一の型 |

### タプル / Tuple

```Rust
// 設定の仕方① : 型の明示あり
let tup: (i32, f64, u8) = (500, 6.4, 1);
// 設定の仕方② : 型の明示なし
let tup = (500, 6.4, 1);

// 値の取出し方① : まとめて
let (x, y, z) = tup;
// 値の取出し方② : 指定して
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;
```

### 配列 / Array

```Rust
// 設定の仕方① : 型/要素数の明示あり
let a: [i32; 5] = [1, 2, 3, 4, 5];
// 設定の仕方② : 型/要素数の明示なし
let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul",
              "Aug", "Sep", "Oct", "Nov", "Dec"];

// 値の取出し方① : まとめて
let (x, y, z) = tup;
// 値の取出し方② : 指定して
let first = a[0];
let second = a[1];
```

# 関数 / Function

関数の引数設定

```rust
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

戻り値の設定

※注意！
戻り値にはセミコロン`;`をつけないこと。つけるとエラーになります。

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

# for文

通常利用のサンプル

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

変数が`bool`の場合は、以下のように条件式が不要。
ただし、`bool`以外では（`u32`などでも）エラーとなる。

```rust
fn main() {
    let flag = True;

    if flag {
        println!("number was three");
    }
}
```

インラインでの記述方法

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

// ↓エラーになる記述
// 条件によって代入されるデータ型が異なることはできない
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };
}
```


# Errorの扱い

`panic!`と`Result`の2つがある

`panic!` : エラーを検出したらプログラムは停止
`Result` : エラーを検出してもプログラムは継続

[error_tutorial](../04_basic_concept/error_concept/src/main.rs)
> ../04_basic_concept/error_concept/src/main.rs















