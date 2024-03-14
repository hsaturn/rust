use std::fs::read_to_string;

fn func() -> u32 {
  5
}

fn main() {
  let lines: Vec<_> = read_to_string("charset.14")
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

  println!("jump {} always" , func());
  let row = 0;
  for line in &lines {
    let hexa = line.split(";").next().unwrap().trim_start_matches("0x");
    let bits = i64::from_str_radix(hexa, 16).unwrap();
    println!("{} = {}", hexa, bits);
  }
}
