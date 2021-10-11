fn main() {
    println!("Hello, trait!");

    let c1 = 1;
    let mut c1m = 2;
    let v1 = vec![1,2,3];
    let mut v1m = vec![2,3,4];

    // c1 = 10; <--Error:immutableは書換不可
    c1m = 20;
    // v1[0] = 10;; <--Error:immutableは書換不可
    v1m[0] = 20;

}

fn func1(a: i32, b: &i32, c: &mut i32) {
    println!("a = {} : i32", a);
    println!("b = {} : &i32", b);
    println!("c = {} : &mut i32", c);
}