use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
  let mut seen: HashSet<i32> = HashSet::new();
  let mut sum: i32 = 0;

  let f = File::open("../input").expect("file not found");
  let nums: Vec<i32> = BufReader::new(&f).lines()
    .enumerate()
    .map(|(_, line)| line.unwrap().parse::<i32>().unwrap())
    .collect();

  for num in nums.iter().cycle() {
    sum += num.clone();
    if !seen.insert(sum) {
      println!("{}", sum);
      ::std::process::exit(0);
    }
  }
}