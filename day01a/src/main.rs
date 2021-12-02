use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut count: i32 = 0;
    let mut num: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let new = line.unwrap().parse::<i32>().unwrap();
            if num < new {
                if num != 0 {
                    count = count + 1;
                }
            }
            num = new;
        }
    }
    print!("{}", count);
}
