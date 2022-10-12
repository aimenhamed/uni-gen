use std::fs;
use std::io;
use std::io::prelude::*;
use regex::Regex;

use dirs::home_dir;

const YEAR: &str = "year3";

fn main() {
    let home = home_dir().unwrap();
    let mut term = String::new();
    println!("What term are you generating files for?");

    io::stdin().read_line(&mut term).expect("Failed");

    let term_string: String = match term.trim().parse() {
        Ok(str) => str,
        Err(_) => return,
    };

    println!("Which courses are you generating files for?");

    let mut courses = String::new();
    io::stdin().read_line(&mut courses).expect("Failed");
    let courses: Vec<&str> = courses.split_whitespace().collect();

    println!("Generating courses files for {}.", courses.join(" "));

    let mut zsh_rc = fs::OpenOptions::new()
        .append(true)
        .open(format!("{}/.zshrc", home.to_str().unwrap()))
        .unwrap();

    if let Err(e) = writeln!(zsh_rc, "# {} {}", YEAR, term_string.to_lowercase()) {
       eprintln!("Couldn't write to zshrc: {}", e);
    }
    
    for course in courses {
        let course_path = format!(
            "{}/uni/{}/{}/{}",
            home.to_str().unwrap(),
            YEAR,
            term_string.to_lowercase(),
            course.to_lowercase()
        );

        fs::create_dir_all(&course_path)
            .expect("error");

        let re = Regex::new(r"[A-Za-z]").unwrap();
        let course_number = re.replace_all(course, ""); 
        let alias = format!(
            "alias {}='cd {}'",
            course_number,
            course_path
        );

        if let Err(e) = writeln!(zsh_rc, "{}", alias) {
           eprintln!("Couldn't write to zshrc: {}", e);
        }
    }

    println!("Done!");
}
