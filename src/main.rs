use std::env;
use std::process;

trait Finder {
    fn split_path(&mut self);
    fn list(&self, locations: Option<Vec<u8>>);
    fn find_locations(&self, word: String) -> Vec<u8>;
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
    fn list(&self, locations: Option<Vec<u8>>) {
        match locations {
            Some(l) => {
                let mut index: u8 = 0;
                for place in &self.places {
                    if l.contains(&index) {
                        println!("{}", place);
                    }
                    index += 1;
                }
            }
            None => {
                for place in &self.places {
                    println!("{}", place);
                }
            }
        }
    }
    fn find_locations(&self, word: String) -> Vec<u8> {
        let mut locations = Vec::<u8>::new();
        let mut index: u8 = 0;
        for place in &self.places {
            if place.contains(&word) {
                locations.push(index);
            }
            index += 1;
        }
        return locations;
    }
}

fn usage() {
    println!(
        "
        Pathfinder -- 0.1
        ----------------
        Usage: pathfinder [OPTION]... [SEARCH]...
        h, --help       Display this page and exit
        v, --version    Display the version and exit
        l, --list       List all locations in $PATH (same as find with no keyword)
        f, --find       Find locations in $PATH including search keyword
        "
    );
    process::exit(0);
}

fn version() {
    println!("Version: 0.1");
    process::exit(0);
}

fn arg_parser(args: Vec<String>, finder: Path) {
    if args.len() >= 1 {
        match args[1].as_ref() {
            "--version" | "-v" | "v" => version(),
            "--help" | "-h" | "h" => usage(),
            "--find" | "-f" | "f" => {
                if args.len() >= 3 {
                    let loc = finder.find_locations(args[2].to_string());
                    finder.list(Some(loc));
                } else {
                    finder.list(None);
                }
            }
            "--list" | "l" => finder.list(None),
            _ => process::exit(0),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("No command passed\nUse --help for more info");
        process::exit(0);
    }

    let path = env::var("PATH");
    match path {
        Ok(path) => {
            let mut finder = Path {
                path: path,
                places: Vec::new(),
            };
            finder.split_path();
            arg_parser(args, finder)
        }
        Err(e) => eprint!("{}", e),
    }
}
