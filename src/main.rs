use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::vec::Vec;

fn main() {
    if let Ok(lines) = read_lines("./test.html") {
        let mut stack = Vec::new();
        for line in lines.map_while(Result::ok) {
            // Loop through each line of html
            // add <div> to stack
            // if can't find closing </div>
            if line.contains("<div>") {
                stack.push(line);
            }
            // aka if </div> doesn't match prev <div>
            // show error(?)
            if line.contains("</div>") {}
        }
        // print how many items and each item in stack
        println!("Found {} lines in file", stack.len());
        for item in &stack {
            println!("{item}")
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
