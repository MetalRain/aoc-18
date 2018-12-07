use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
  let f = File::open("input").expect("file not found");
  let sum = BufReader::new(&f).lines()
    .enumerate()
    .fold(0, |acc, (_, line)| acc + line.unwrap().parse::<i32>().unwrap());
  println!("{}", sum)
}