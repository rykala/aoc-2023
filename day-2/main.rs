use std::fs::read_to_string;
use std::cmp::max;

static RED_CUBES: i32 = 12;
static GREEN_CUBES: i32 = 13;
static BLUE_CUBES: i32 = 14;

#[derive(Copy, Clone)]
struct Round {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

struct Game {
    game_number: i32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Cube {
    color: String,
    count: i32,
}

fn main() {
    let result_part_1 = read_input()
        .into_iter()
        .map(parse_game)
        .filter(is_valid_game)
        .map(|x| x.game_number)
        .sum::<i32>();
    println!("part_1: {}", result_part_1);

    let result_part_2 = read_input()
        .into_iter()
        .map(parse_game)
        .map(|x| get_minimum_cubes_needed_to_play(&x))
        .sum::<i32>();
    println!("part_2: {}", result_part_2);
}

fn get_minimum_cubes_needed_to_play(game: &Game) -> i32 {
    let mut red_cubes = 0;
    let mut green_cubes = 0;
    let mut blue_cubes = 0;

    game.rounds.iter().for_each(|x| {
        red_cubes = max(red_cubes, x.red_cubes);
        green_cubes = max(green_cubes, x.green_cubes);
        blue_cubes = max(blue_cubes, x.blue_cubes);
    });
    return red_cubes * green_cubes * blue_cubes;
}

fn parse_game(unparsed_game: String) -> Game {
    let game_number = parse_game_number(&unparsed_game);
    let rounds = parse_rounds(&unparsed_game);
    return Game {
        game_number,
        rounds,
    };
}

fn parse_game_number(unparsed_game: &str) -> i32 {
    return unparsed_game
        .split(":")
        .nth(0)
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_rounds(unparsed_game: &str) -> Vec<Round> {
    return unparsed_game
        .split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(parse_round)
        .collect();
}

fn parse_round(unparsed_round: &str) -> Round {
    let cubes = unparsed_round
        .split(",")
        .map(str::trim)
        .map(|x| x.split(" ").collect::<Vec<_>>())
        .map(|x| Cube {
            count: x.get(0).unwrap().parse().unwrap(),
            color: x.get(1).unwrap().to_string(),
        })
        .collect::<Vec<Cube>>();
    let red_cubes = parse_cubes(&cubes, "red");
    let green_cubes = parse_cubes(&cubes, "green");
    let blue_cubes = parse_cubes(&cubes, "blue");
    return Round {
        red_cubes,
        green_cubes,
        blue_cubes,
    };
}

fn parse_cubes(cubes: &Vec<Cube>, cube_color: &str) -> i32 {
    return match cubes.iter().find(|x| x.color == cube_color) {
        Some(x) => x.count,
        None => 0,
    };
}

fn is_valid_game(game: &Game) -> bool {
    return game.rounds.iter().all(|x| {
        x.red_cubes <= RED_CUBES && x.green_cubes <= GREEN_CUBES && x.blue_cubes <= BLUE_CUBES
    });
}

fn read_input() -> Vec<String> {
    return read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
