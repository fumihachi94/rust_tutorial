use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();  
    println!("{:#?}", paths);
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}
