// use std::{fs, io};
// use std::path::Path;
// use glob::glob;
// use std::time::SystemTime;
// use chrono::{DateTime, TimeZone, Utc, Local, Duration};
use std::fs;
use glob::glob;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{TimeZone, Local};
use std::process::{Command, Child};

pub fn main(){
    // let root = Path::new("./test");
    // let _a = serching(root);

    let root = "./test/**/*.*";
    search_files(root);

    command_test("./src/Xpeng_G3_OTA.png");
}

/// Open file by default application
fn file_open(open_com: &str, file_path: &str) -> Child{
    return Command::new(open_com)
        .arg(file_path)
        .spawn()
        .expect("Failed to execute file open process.");
}

#[cfg(target_os = "linux")]
fn command_test(file_path: &str){
    println!("This is Linux os.");
    let mut output = file_open("xdg-open", file_path);
}

#[cfg(target_os = "windows")]
fn command_test(file_path: &str){
    println!("This is Windows os.");
    let mut output = file_open("start", file_path);
}

#[cfg(target_os = "macos")]
fn command_test(file_path: &str){
    println!("This is maxos os.");
    let mut output = file_open("open", file_path);
}

fn search_files(dir: &str){
    for entry in glob(dir).expect("Failed to read glob patern"){
        match entry{
            Ok(path) => {
                let mod_sys_time: SystemTime = fs::metadata(path.as_path()).
                    unwrap().
                    modified().
                    unwrap();
                if let Ok(epoch) = mod_sys_time.duration_since(UNIX_EPOCH){
                    let dt_local = Local.timestamp(epoch.as_secs() as i64, epoch.subsec_nanos());
                    println!("{} --- {}", 
                        path.display(),
                        dt_local.format("%Y-%m-%d %H:%M:%S (JST)").to_string());
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}


// fn serching(dir: &Path) -> io::Result<()> {
//     for entry in fs::read_dir(dir)?{
//         let path = entry?.path();
//         if path.is_dir(){
//             println!("Dire: {}", path.display());
//             let _rslt = serching(path.as_path());
//         }else{
//             println!("File: {}", path.display());
//         }
//     }
//     Ok(())
// }



fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}