use std::io::{stdout, Write, stdin};
use std::fmt::Display;

pub fn process_user_input() -> Option<String> {
  let mut user_input = String::new();
  print!("> ");
  stdout().flush().unwrap();
  if stdin().read_line(&mut user_input).is_err() {
    println!("input error!");
    return None
  };
  Some(user_input.trim().to_string())
}

pub fn process_select_one_of<T: Display + Clone>(variants: Vec<T>) -> Option<T> {
  if variants.len() == 0 {
    return None
  }
  if variants.len() == 1 {
    println!("selection of one variant performed automatically: {}", variants[0]);
    return Some(variants[0].clone())
  }
  println!("select one of:");
  for i in 0..variants.len() {
    println!("  [{}] {}", i, variants[i]);
  }
  loop {
    let user_selection_str = match process_user_input() {
      Some(str) => str,
      None => {
        println!("you should select exactly one (write a number)!");
        continue;
      }
    };
    let user_selection_id = match user_selection_str.parse::<usize>() {
      Ok(res) => res,
      Err(_) => {
        println!("you should write number!");
        continue;
      }
    };
    if user_selection_id > variants.len() {
      println!("there's no such variant");
      continue;
    }
    return Some(variants[user_selection_id].clone())
  }
}
