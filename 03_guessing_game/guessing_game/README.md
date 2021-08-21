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

