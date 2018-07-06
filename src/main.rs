use std::io::{self, BufRead};
extern crate regex;

use regex::Regex;

const OPENING_BRACES: [&str; 3] = ["(", "{", "["];

fn main() {
  println!("Braces, input braces string!");
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    match line {
      Ok(line) => {
        let regex = Regex::new(r"[\{\[\(\}\]\)]+").unwrap();
        if regex.is_match(&line) {
          println!("All braces match: {}", all_braces_match(&line))
        } else {
          println!("{}", "Please input a braces string")
        }
      }
      _ => println!("Please input a braces string"),
    }
  }
}

fn all_braces_match(braces_string: &str) -> bool {
  let braces: Vec<&str> = braces_string
    .split("")
    .filter(|&character| character != "")
    .collect();
  let mut opening_braces_stack: Vec<&str> = Vec::new();

  for brace in braces {
    let is_opening_brace = OPENING_BRACES
      .iter()
      .any(|&opening_brace| opening_brace == brace);

    if is_opening_brace {
      opening_braces_stack.push(brace);
    } else {
      let poped_brace = opening_braces_stack.pop().unwrap_or("");
      let do_not_match = !is_closing_brace_of(&poped_brace, &brace);
      if do_not_match {
        return false;
      }
    }
  }

  opening_braces_stack.len() == 0
}

fn is_closing_brace_of(opening_brace: &str, closing_brace: &str) -> bool {
  let is_match_1 = opening_brace == "(" && closing_brace == ")";
  let is_match_2 = opening_brace == "[" && closing_brace == "]";
  let is_match_3 = opening_brace == "{" && closing_brace == "}";
  is_match_1 || is_match_2 || is_match_3
}
