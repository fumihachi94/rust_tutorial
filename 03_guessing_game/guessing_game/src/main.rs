//! # Guessing Game
//!
//! Rust公式チュートリアルをほぼそのまま
//! - [Programming a Guessing Game - The Rust Programming Language](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
//!
//! `cargo doc`用のコメントはMarkdown形式で記述する模様
//! ```sh
//! $ cargo doc --open
//! ```
//! ドキュメントの記載方法について：
//! - [Crates.ioにクレートを公開する - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/ch14-02-publishing-to-crates-io.html)

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Guessing Game main function
/// メイン関数
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;}
        };
        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
