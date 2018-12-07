use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
  let filename = "input";

  let f = File::open(filename).expect("file not found");
  let file = BufReader::new(&f);
  let mut sum = 0;

  for (_, line) in file.lines().enumerate() {
    let l = line.unwrap();
    let num = l.parse::<i32>().unwrap();
    sum += num;
  }
  println!("{}", sum)
}