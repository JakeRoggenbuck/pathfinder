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
    fn search(&self, args: Vec<String>, enumerate: bool);
    fn enumerate_print(&self, number: &mut i32, literal: String, enumerate: bool);
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
    fn enumerate_print(&self, number: &mut i32, literal: String, enumerate: bool) {
        if enumerate {
            println!("{}\t{}", *number, literal);
            *number += 1;
        } else {
            println!("{}", literal);
            *number += 1;
        }
    }
    fn list(&self, locations: Option<Vec<u8>>, enumerate: bool) {
        let mut number: i32 = 0;
        match locations {
            Some(l) => {
                let mut index: u8 = 0;
                for place in &self.places {
                    if l.contains(&index) {
                        self.enumerate_print(&mut number, place.to_string(), enumerate);
                    }
                    index += 1;
                }
            }
            None => {
                for place in &self.places {
                    self.enumerate_print(&mut number, place.to_string(), enumerate);
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
    fn search(&self, args: Vec<String>, enumerate: bool) {
        let phrase = if args.len() >= 3 {
            Some(self.find_locations(args[2].to_string()))
        } else {
            None
        };
        self.list(phrase, enumerate);
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
            "--version" | "version" | "-v" | "v" => version(),
            "--help" | "help" | "-h" | "h" => usage(),
            "--add" | "add" | "-a" | "a" => {
                if args.len() >= 3 {
                    finder.add(args[2].to_owned());
                }
            }
            "--find" | "find" | "-f" | "f" => finder.search(args, false),
            "--list" | "list" | "-l" | "l" => finder.list(None, false),
            "--purge" | "purge" | "-p" | "p" => finder.purge(),
            "--number" | "number" | "-n" | "n" => finder.search(args, true),
            _ => {
                println!("Command {} not found", args[1]);
                process::exit(0)
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_test() {
        let mut finder = Finder {
            path: "/one/word:/two:/words".to_owned(),
            places: Vec::new(),
        };
        finder.split_path();
        assert_eq!(finder.places, vec!["/one/word", "/two", "/words"]);
    }

    #[test]
    fn find_locations_test() {
        let mut finder = Finder {
            path: "this:that:/ya/know:/everything".to_owned(),
            places: Vec::new(),
        };
        finder.split_path();
        assert_eq!(finder.find_locations("a".to_owned()), vec![1, 2]);
        assert_eq!(finder.find_locations("y".to_owned()), vec![2, 3]);
        assert_eq!(finder.find_locations("t".to_owned()), vec![0, 1, 3]);
    }
}
