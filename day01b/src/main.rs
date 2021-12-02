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
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let new = line.unwrap().parse::<i32>().unwrap();
            if a != 0 && b != 0 && c != 0 && a < new {
                count = count + 1;
            }
            let (mut a, mut b, mut c) = (b, c, new);
        }
    }
    print!("{}", count);
}
