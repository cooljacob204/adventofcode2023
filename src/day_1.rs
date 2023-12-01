use regex::Regex;

pub fn day_1() {
  part_1();
  part_2();
}

fn part_1() {
  let input = parse_input();

  let mut sum = 0;

  for line in input {
    let mut first_num: Option<i32> = None;
    let mut last_num: Option<i32> = None;

    for char in line.chars() {
      let num = char.to_digit(10);

      if num.is_none() {
        continue;
      }

      let num = num.unwrap() as i32;

      if first_num.is_none() {
        first_num = Some(num);
      } else {
        last_num = Some(num);
      }
    }

    if first_num.is_some() && last_num.is_some() {
      sum += (first_num.unwrap() * 10) + last_num.unwrap();
    } else if first_num.is_some() {
      sum += first_num.unwrap() * 10 + first_num.unwrap();
    }
  }

  println!("Day 1, part 1: {}", sum);
}

fn part_2(){
  let re_first: Regex = Regex::new(r"^.*?(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
  let re_last: Regex = Regex::new(r"^.*(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
  let input = parse_input();

  let mut sum = 0;

  for line in input {
    let first = re_first.captures(line.as_str()).unwrap().get(1);
    let last = re_last.captures(line.as_str()).unwrap().get(1);

    let first_num = string_to_num(first.unwrap().as_str());
    let last_num = string_to_num(last.unwrap().as_str());
    sum += first_num * 10 + last_num;
  }

  println!("Day 1, part 2: {}", sum);
}

fn string_to_num(string: &str) -> i32 {
  match string {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    _ => string.parse::<i32>().unwrap()
  }
}

fn parse_input() -> Vec<String> {
    let input = include_str!("./inputs/day_1.txt");
    let mut lines: Vec<String> = Vec::new();
    for line in input.lines() {
        lines.push(line.to_string());
    }
    lines
}