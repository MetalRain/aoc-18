use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
  let f = File::open("input").expect("file not found");
  let file = BufReader::new(&f);
  let mut sum = 0;

  for (_, line) in file.lines().enumerate() {
    let num = line.unwrap().parse::<i32>().unwrap();
    sum += num;
  }
  println!("{}", sum)
}