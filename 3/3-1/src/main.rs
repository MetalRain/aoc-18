extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use regex::Regex;

type Patch = (i32, i32, i32, i32);
type Overlaps = HashMap<(i32,i32), i32>;

fn as_int(m: regex::Match) -> i32 {
  return m.as_str().parse::<i32>().unwrap()
}

fn parse_line(s: String, re: &Regex) -> Patch {
  let parts = re.captures(&s).unwrap();
  return (
    as_int(parts.get(1).unwrap()),
    as_int(parts.get(2).unwrap()),
    as_int(parts.get(3).unwrap()),
    as_int(parts.get(4).unwrap())
  );
}

fn fill_tiles(patch: Patch, mut tiles: Overlaps) -> Overlaps {
  let (x, y, w, h) = patch;
  for dx in x..(x+w) {
    for dy in y..(y+h) {
      let count = tiles.entry((dx,dy)).or_insert(0);
      *count += 1;
    }
  }
  return tiles
}

fn main() {
  let f = File::open("../input").expect("file not found");
  let line_re: Regex = Regex::new(r"^#\d+ @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

  let overlaps = BufReader::new(&f).lines()
    .enumerate()
    .map(|(_, line)| parse_line(line.unwrap(), &line_re))
    .fold(HashMap::new(), |m, patch| fill_tiles(patch, m))
    .iter()
    .fold(0, |a, (_, v)| if v >= &2 { a + 1 } else { a });

  println!("{}", overlaps);
}