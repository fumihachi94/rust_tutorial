# struct

[Using Structs to Structure Related Data - The Rust Programming Language](https://doc.rust-lang.org/book/ch05-00-structs.html)


<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [struct](#struct)
  - [使用例1: 通常使用（別ファイルから読出す）](#使用例1-通常使用別ファイルから読出す)
  - [使用例2: 同一パラメータを持つ構造体](#使用例2-同一パラメータを持つ構造体)
  - [使用例3: 構造体を関数の引数にする](#使用例3-構造体を関数の引数にする)
  - [使用例4: 構造体を標準出力する](#使用例4-構造体を標準出力する)

<!-- /code_chunk_output -->


## 使用例1: 通常使用（別ファイルから読出す）

- src/main.rs

```rust
mod struct_tutorial;
pub use struct_tutorial::struct_tutorial::*;

fn main() {
    let user1 = set_user(String::from("user1"),String::from("user1@mail.com"));
    show_userinfo(user1);
}
```

- src/struct_tutorial.rs

```rust
pub mod struct_tutorial{
    
    #[derive(Debug)]
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    pub fn set_user(your_username: String, email: String) -> User {
        return User {
            username: your_username,
            email, // パラメータ名が同一の場合は省略可能
            active: true,
            sign_in_count: 1,
        };
    }

    pub fn show_userinfo(user_info: User){
        println!("{:#?}",user_info);
    }

}
```

出力

```sh
> cargo run
User {
    username: "user1",
    email: "user1@mail.com",
    sign_in_count: 1,
    active: true,
}
```

## 使用例2: 同一パラメータを持つ構造体

- src/main.rs

```rust
mod struct_tutorial2;
pub use struct_tutorial2::struct_tutorial2::create_someuser;

fn main() {
    create_someuser();
}
```
- src/struct_tutorial2.rs

```rust
pub mod struct_tutorial2 {
    #[derive(Debug)]
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

pub fn create_someuser() {
        let user_a = User {
            email: String::from("user_a@mail.com"),
            username: String::from("user_a"),
            active: true,
            sign_in_count: 2,
        };
        let user_b = User {
            email: String::from("user_b@mail.com"),
            username: String::from("user_b"),
            ..user_a // ★他パラメータは”user_a”と同様の値がセットされる
        };

        println!("{:#?}", user_a);
        println!("{:#?}", user_b);
    }
}
```

出力

```sh
> cargo run
User {
    username: "user_a",
    email: "user_a@mail.com",
    sign_in_count: 2,
    active: true,
}
User {
    username: "user_b",
    email: "user_b@mail.com",
    sign_in_count: 2,
    active: true,
}
```

## 使用例3: 構造体を関数の引数にする

他の引数と同様の記述で、構造体を引数に指定することが出来ます。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

## 使用例4: 構造体を標準出力する

通常の`println!("{}", struct)`という方法ではエラーになってしまします。
結論というと、構造体定義の前に`#[derive(debug)]`というアノテーションをつけ
`println!("{:?}", struct)`もしくは`println!("{:#?}", struct)`で出力出来ます。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1); // <-#を付けると階層化して出力してくれる
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }
}
```



