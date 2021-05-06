use std::env;

trait Finder {
    fn split_path(&mut self);
    fn list(&self);
}

struct Path {
    path: String,
    places: Vec<String>,
}

impl Finder for Path {
    fn split_path(&mut self) {
        let split = self.path.split(":");
        let mut vec = Vec::<String>::new();
        for x in split {
            if !vec.contains(&x.to_string()) {
                vec.push((&x).to_string());
            }
        }
        self.places = vec;
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
            let mut finder = Path {
                path: path,
                places: Vec::new(),
            };
            finder.split_path();
            finder.list();
        }
        Err(e) => eprint!("{}", e),
    }
}
