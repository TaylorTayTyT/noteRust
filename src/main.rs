use std::{env, path::{Path, PathBuf}};
fn main() {
    let a = env::current_dir().unwrap();
    let b = a.join("new File");
    println!("{:?}", b);
    //println!("{:?}", env::current_dir().unwrap().display().to_string());
}
