use regex::Regex;

pub fn day_2() {
  part_1();
  part_2();
}

fn part_1() {
  let input = parse_input();

  let mut sum = 0;

  for game in input {
    let mut largest_green = 0;
    let mut largest_red = 0;
    let mut largest_blue = 0;

    for hand in game.hands {
      if hand.red.is_some() {
        let red = hand.red.unwrap();

        if red > largest_red {
          largest_red = red;
        }
      }

      if hand.blue.is_some() {
        let blue = hand.blue.unwrap();

        if blue > largest_blue {
          largest_blue = blue;
        }
      }

      if hand.green.is_some() {
        let green = hand.green.unwrap();

        if green > largest_green {
          largest_green = green;
        }
      }
    }

    if largest_green > 0 && largest_red > 0 && largest_blue > 0 &&
       largest_green <= 13 && largest_red <= 12 && largest_blue <= 14 {

      sum += game.id;
    }
  }

  println!("Day 2, part 1: {}", sum);
}

fn part_2() {
    let input = parse_input();

    let mut sum = 0;

    for game in input {
      let mut minimum_green: i32 = 1;
      let mut minimum_red: i32 = 1;
      let mut minimum_blue: i32 = 1;


      for hand in game.hands {
        if hand.red.is_some() {
          let red = hand.red.unwrap();

          if red > minimum_red  {
            minimum_red = red;
          }
        }

        if hand.blue.is_some() {
          let blue = hand.blue.unwrap();

          if blue > minimum_blue {
            minimum_blue = blue;
          }
        }

        if hand.green.is_some() {
          let green = hand.green.unwrap();

          if green > minimum_green {
            minimum_green = green;
          }
        }
      }

      sum += minimum_green * minimum_red * minimum_blue;
    }

    println!("Day 2, part 2: {}", sum);
}

fn parse_input() -> Vec<Game> {
  let input = include_str!("./inputs/day_2.txt");
  let mut lines: Vec<Game> = Vec::new();

  for line in input.lines() {
    lines.push(parse_game(line));
  }

  lines
}

fn parse_game(line: &str) -> Game {
  let re = Regex::new(r"Game (?<game_id>\d*): (?<hands>.*$)").unwrap();
  let captures = re.captures(line).unwrap();
  let game_id = captures.name("game_id").unwrap().as_str().parse::<i32>().unwrap();
  let hands = captures.name("hands").unwrap().as_str().split(";");

  let red_re = Regex::new(r"(?<red>\d*) red").unwrap();
  let blue_re = Regex::new(r"(?<blue>\d*) blue").unwrap();
  let green_re = Regex::new(r"(?<green>\d*) green").unwrap();

  let hands = hands.into_iter().map(|hand| {
    let red_captures = red_re.captures(hand);
    let blue_captures = blue_re.captures(hand);
    let green_captures = green_re.captures(hand);
    let red = red_captures.and_then(|r| r.name("red").and_then(|r| r.as_str().parse::<i32>().ok()));
    let blue = blue_captures.and_then(|r| r.name("blue").and_then(|r| r.as_str().parse::<i32>().ok()));
    let green = green_captures.and_then(|r| r.name("green").and_then(|r| r.as_str().parse::<i32>().ok()));

    Hand {
      red: red,
      blue: blue,
      green: green
    }
  }).collect::<Vec<Hand>>();

  Game {
    id: game_id,
    hands
  }
}

struct Game {
  id: i32,
  hands: Vec<Hand>
}

struct Hand {
  red: Option<i32>,
  blue: Option<i32>,
  green: Option<i32>
}