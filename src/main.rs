use std::fs;
use std::io;

use dirs::home_dir;

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

    for course in courses {
        fs::create_dir_all(format!(
            "{}/uni/year3/{}/{}",
            home.to_str().unwrap(),
            term_string.to_lowercase(),
            course.to_lowercase()
        ))
        .expect("error");
    }

    println!("Done!");
}
