use std::env;

fn split_path(path: &String) -> Vec<&str> {
    let split = path.split(":");
    let vec: Vec<&str> = split.collect();
    return vec;
}

fn main() {
    let path = env::var("PATH");
    match path {
        Ok(path) => {
            let vec = split_path(&path);
            println!("{:?}", vec);
        }
        Err(e) => eprint!("{}", e),
    }
}
