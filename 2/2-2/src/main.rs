use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn diff_max_two(s: &Vec<char>, t: &Vec<char>) -> i32 {
  let mut diff = 0;
  let z = s.iter().zip(t.iter());
  for (a, b) in z {
    if a != b {
      diff += 1;
    }
    if diff > 1 {
      return diff;
    }
  }
  return diff;
}

fn filter_diff(s: &Vec<char>, t: &Vec<char>) -> Vec<char> {
  return s.iter().zip(t.iter())
    .fold(Vec::new(), |mut vec, (a, b)| if a != b { vec } else { vec.push(*a); vec })
}

fn main() {
  let f = File::open("../input").expect("file not found");
  let rows: Vec<(usize, Vec<char>)> = BufReader::new(&f).lines()
    .enumerate()
    .map(|(num, line)| (num, line.unwrap().chars().collect()))
    .collect();

  for &(i, ref s) in rows.iter() {
    for &(j, ref s2) in rows.iter() {
      if j > i {
        let diff = diff_max_two(s, s2);
        if diff == 1 {
          let same_chars = filter_diff(s, s2);
          let result: String = same_chars.iter().map(|c| *c).collect();
          println!("{}", result);
          ::std::process::exit(0);
        }
      }
    }
  }
}