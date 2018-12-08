use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn scores(s: String) -> (i32, i32) {
  let mut times: HashMap<char, i32> = HashMap::new();
  for c in s.chars() {
    let count = times.entry(c).or_insert(0);
    *count += 1;
  }
  return times.iter()
    .fold((0, 0),
      |(acc_twos, acc_threes), (_, char_count)| 
        ( acc_twos + if acc_twos == 0 && *char_count == 2 { 1 } else { 0 },
          acc_threes + if acc_threes == 0 && *char_count == 3 { 1 } else { 0 }
        ));
}

fn main() {
  let f = File::open("../input").expect("file not found");
  let sums = BufReader::new(&f).lines()
    .enumerate()
    .map(|(_, line)| scores(line.unwrap()))
    .fold((0, 0),
      |(acc_twos, acc_threes), (twos, threes)| (acc_twos + twos, acc_threes + threes));
  let (twos, threes) = sums;
  println!("{}", twos * threes);
}





