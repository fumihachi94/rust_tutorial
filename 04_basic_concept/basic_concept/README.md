# Rust basic concept

[Common Programming Concepts - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)


<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [Rust basic concept](#rust-basic-concept)
- [Functions](#functions)
  - [f1. 引数あり＋戻値あり](#f1-引数あり戻値あり)
  - [f2. 引数あり＋戻値なし](#f2-引数あり戻値なし)
  - [f3. 引数なし＋戻値あり](#f3-引数なし戻値あり)
  - [f4. 引数なし＋戻値なし](#f4-引数なし戻値なし)
- [if](#if)
- [loop](#loop)
- [while](#while)
- [for](#for)

<!-- /code_chunk_output -->


# Functions

|引数 |     |あり   |なし   |
|--   |--   |--     |--     |
|戻値 |あり |[f1](#f1-引数あり戻値あり) |[f3](#f3-引数なし戻値あり) |
|^    |なし |[f2](#f2-引数あり戻値なし) |[f4](#f4-引数なし戻値なし) |

## f1. 引数あり＋戻値あり

```rust
fn main(){
    // 戻り値が1つ
    let a = func_1a(123);
    println!("{}", a);
    
    // 戻り値が複数
    // タプルで返す
    let b = func_1b(123,456);
    println!("{} {}", b.0, b.1);
    // 2つの変数で返す
    let (c,d) = func_1b(123,456);
    println!("{} {}", c, d);
}

fn func_1a(x: i32) -> i32 {
    x*2+1
}

fn func_1b(x: i32, y: i32) -> (i32, i32) {
    return (x*2+1, y*3)
}
```

## f2. 引数あり＋戻値なし

```rust
fn func_2(x: i32) {
    let y = x*2+1;
    println!("y=2x+1: x={}, y={}", x, y);
}
```

## f3. 引数なし＋戻値あり

```rust
fn func_3() -> i32 {
    let x = 2;
    x*2+1
}
```

## f4. 引数なし＋戻値なし

```rust
fn func_4() {
    let x = 2;
    println!("x={}", x);
}
```

# if

```rust
fn main() {
    let flag : bool = true;
    let num = 6;

    if flag {
        if num % 3 == 0 {
            println!("number is divisible by 3");
        } else if num % 2 == 0 {
            println!("number is divisible by 2");
        } else{
            println!("number is not divisible by 3, or 2");
        }
    }

    // describe inline
    let dst = if num % 3 == 0 {1} else {0};
    println!("dst = {}", dst);
}
```

# loop

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

# while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

# for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```
