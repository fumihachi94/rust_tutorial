/// Basic Copncept Tutoprial  
/// [Common Programming Concepts - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
///   
/// Notes for myself. Writing somthieng I noticed.  
/// @fumihachi : 2021/08/28
fn main() {
    println!("== Rust basic concept ==");

    var_and_mut();

    data_types();

    let x = function_1(5);
    println!("x = {}", x);

    let y = function_2(2);
    println!("a = {}, b = {}", y.0, y.1);

    func_if();
}

/// Variables and Mutability  
/// [Variables and Mutability - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
fn var_and_mut(){
    println!("== 1. Variables and Mutability ==");
    
    // immutable variable
    let _a_imut = 5;
    /*     
    a_imut = 6; <-- This substitution occurs error!
    */

    // mutable variable
    let mut _a_mut = 5;
    _a_mut = 6;

    // constant
    const A_CONST: u32 = 100_000;
    println!("# constant\n A_CONST = {}", A_CONST);

    // shadowing
    let _b = 5;
    let _b = _b*2;
    let _b = _b+2;
    println!("# shadowing\n b = {}", _b);

    let _b = "change to string (immutable only)";
    println!(" b = {}", _b);

    /* Can't change type at mutable variable 
    let mut _b_mut = 5;
    _b_mut = "change to string (immutable only)";
    */

    let _spaces_imut = " ";
    println!("# word length\n len_imut = {}", _spaces_imut.len());
    let mut _spaces_mut = "  ";
    println!("# word length\n len_mut = {}", _spaces_mut.len());
}

/// Data Types
/// [Data Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch03-02-data-types.html)
fn data_types(){
    println!(r#"== 2. Data Types =="#);

    let _u32_dec_a : u32 = "255".parse().expect("Not a number!");
    let _u32_hex_a : u32 = 0xFF;
    let _u32_oct_a : u32 = 0o77;
    let _u32_bin_a : u32 = 0b1111_1111;
    let _u32_byt_a : u8 = b'A';


    println!("# type");
    println!("_u32_dec = {} : 255 ", _u32_dec_a);
    println!("_u32_hex = {} : 0xFF ", _u32_hex_a);
    println!("_u32_oct = {} : 0o77 ", _u32_oct_a);
    println!("_u32_bin = {} : 0b1111_1111 ", _u32_bin_a);
    println!("_u32_byt = {} : b'A' (u8 only) ", _u32_byt_a);
}

fn function_1(x: i32) -> i32 {
    println!(r#"== 3. Function1 : Argument & Return value =="#);
    x*2+1
}

fn function_2(x: i32) -> (i32, i32) {
    println!(r#"== 3. Function2 : Argument & Return value =="#);
    return (x+1, x*2);
}

fn func_if(){
    println!("== 4. if Expressions ==");
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