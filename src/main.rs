use std::env;

trait Finder {
    fn split_path(&self) -> Vec<&str>;
    fn list(&self);
}

struct Path {
    path: String,
    places: Vec<&'static str>
}

impl Finder for Path {
    fn split_path(&self) -> Vec<&str> {
        let split = self.path.split(":");
        let vec: Vec<&str> = split.collect();
        return vec;
    }
    fn list(&self) {
        for place in &self.places {
            println!("{}", place);
        }
    }
}


fn main() {
    let path = env::var("PATH");
    match path {
        Ok(path) => {
            let finder = Path {
                path: path,
                places: Vec::new(),
            };
            finder.split_path();
            finder.list();
        }
        Err(e) => eprint!("{}", e),
    }
}
