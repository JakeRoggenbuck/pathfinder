use std::env;
use std::path::Path;
use std::process;
use std::process::Command;

trait PathFinder {
    fn split_path(&mut self);
    fn list(&self, locations: Option<Vec<u8>>, enumerate: bool);
    fn find_locations(&self, word: String) -> Vec<u8>;
    fn spawn(&self);
    fn add(&self, location: String);
    fn purge(&self);
}

struct Finder {
    path: String,
    places: Vec<String>,
}

impl PathFinder for Finder {
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
    fn list(&self, locations: Option<Vec<u8>>, enumerate: bool) {
        match locations {
            Some(l) => {
                let mut index: u8 = 0;
                for place in &self.places {
                    if l.contains(&index) {
                        if enumerate {
                            println!("{}\t{}", index, place);
                        } else {
                            println!("{}", place);
                        }
                    }
                    index += 1;
                }
            }
            None => {
                let mut index: u8 = 0;
                for place in &self.places {
                    if enumerate {
                        println!("{}", place);
                    } else {
                        println!("{}\t{}", index, place);
                    }
                    index += 1;
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
    fn spawn(&self) {
        // Open a new instance of bash on top of the program
        Command::new("/bin/bash")
            .spawn()
            .expect("Failed to execute command");

        // Wait for a long time to let the new bash prompt exist a while
        let mut child = Command::new("sleep").arg("infinity").spawn().unwrap();
        let _result = child.wait().unwrap();
    }
    fn add(&self, location: String) {
        println!("Adding {} to $PATH", location);
        // Checks if the file location exists
        if Path::new(&location).exists() {
            // Set the environmental variable, same as PATH=location:$PATH
            env::set_var(
                "PATH",
                format!("{}:{}", location, env::var("PATH").unwrap()),
            );

            self.spawn();
        } else {
            eprintln!("The location {}, does not exist", location);
        }
    }
    fn purge(&self) {
        env::set_var("PATH", "UNSET");
        self.spawn();
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
        a, --add        Add location to the $PATH, and open a new bash prompt
        p, --purge      Unsets everything in $PATH, it will be empty
        n, --number     List locations in $PATH with numbers on side
        "
    );
    process::exit(0);
}

fn version() {
    println!("Version: 0.1");
    process::exit(0);
}

fn arg_parser(args: Vec<String>, finder: Finder) {
    if args.len() >= 1 {
        match args[1].as_ref() {
            "--version" | "-v" | "v" => version(),
            "--help" | "-h" | "h" => usage(),
            "--add" | "-a" | "a" => {
                if args.len() >= 3 {
                    finder.add(args[2].to_owned());
                }
            }
            "--find" | "-f" | "f" => {
                if args.len() >= 3 {
                    let loc = finder.find_locations(args[2].to_string());
                    finder.list(Some(loc), false);
                } else {
                    finder.list(None, false);
                }
            }
            "--list" | "l" => finder.list(None, false),
            "--purge" | "p" => finder.purge(),
            "--number" | "-n" | "n" => {
                if args.len() >= 3 {
                    let loc = finder.find_locations(args[2].to_string());
                    finder.list(Some(loc), false);
                } else {
                    finder.list(None, false);
                }
            }
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
            let mut finder = Finder {
                path: path,
                places: Vec::new(),
            };
            finder.split_path();
            arg_parser(args, finder)
        }
        Err(e) => eprint!("{}", e),
    }
}
