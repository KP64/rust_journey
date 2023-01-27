#![feature(const_trait_impl, const_cmp)]

use rand::Rng;
use std::{cmp::Ordering, io, ops::RangeInclusive};

const OBSTACLE_DISPLAY: &str = "🔥";
const FREE_SPACE_DISPLAY: &str = "_";

const COURSE_LEN: usize = 100;
const DICE_RANGE: RangeInclusive<usize> = 1..=6;

fn main() {
    let course = make_course(COURSE_LEN);
    // the players place in the course
    let mut place = 0;
    let mut first_roll = true;

    // pass in length of course to avoid printing out the current place of the player
    println!("🎬 {:?}", make_board_display(course.len(), &course));

    loop {
        // 1. roll the dice
        // 2. find the next place in the course for the player
        // 3. move the player to the next space
        // 4. print out the board

        let roll = roll_two_dice();
        let roll = roll.0 + roll.1;

        let turn = PlayerTurn::new(roll, place);

        println!("⏭ {turn:?}");

        let next_place = match find_next_place(&turn, COURSE_LEN, first_roll) {
            NextPlace::GameWon => {
                println!("🏆 Finished the course!");
                break;
            }
            NextPlace::Place(p) => p,
        };

        place = move_player_to_next_place(&course, next_place);

        let snapshot = make_board_display(place, &course);
        println!("🎯 {snapshot:?}");
        first_roll = false;
        println!("======");
    }
}

fn make_board_display(place: usize, course: &[Space]) -> Vec<String> {
    course
        .iter()
        .enumerate()
        .map(|(i, spot)| {
            if i == place {
                format!("+{i}")
            } else {
                match spot {
                    Space::Obstacle(_) => String::from(OBSTACLE_DISPLAY),
                    Space::FreeSpace => String::from(FREE_SPACE_DISPLAY),
                }
            }
        })
        .collect()
}

fn make_course(len: usize) -> Vec<Space> {
    (0..len)
        .map(|_| {
            if rand::thread_rng().gen_bool(0.3) {
                Space::FreeSpace
            } else {
                let penalty = rand::thread_rng().gen_range(2..=3);
                Space::Obstacle(penalty)
            }
        })
        .collect()
}

const fn hit_obstacle_next_place(place: usize, penalty: usize) -> usize {
    let tmp = place - penalty;
    match tmp.cmp(&0) {
        Ordering::Equal | Ordering::Greater => tmp,
        Ordering::Less => 0,
    }
}

///
/// Blocks for user input and rolls the die.
///
fn roll_two_dice() -> (usize, usize) {
    println!("🎲🎲 Roll two dice…");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    (
        rand::thread_rng().gen_range(DICE_RANGE),
        rand::thread_rng().gen_range(DICE_RANGE),
    )
}

#[derive(Debug)]
struct PlayerTurn {
    roll: usize,
    current_place: usize,
}

impl PlayerTurn {
    const fn new(roll: usize, current_place: usize) -> Self {
        Self {
            roll,
            current_place,
        }
    }
}

enum NextPlace {
    Place(usize),
    GameWon,
}

#[derive(Clone, Copy, Debug)]
enum Space {
    Obstacle(usize),
    FreeSpace,
}

const fn find_next_place(turn: &PlayerTurn, course_len: usize, first_roll: bool) -> NextPlace {
    let next_place = turn.current_place + turn.roll;

    if next_place >= course_len {
        NextPlace::GameWon
    } else if first_roll {
        NextPlace::Place(turn.roll - 1)
    } else {
        NextPlace::Place(next_place)
    }
}

fn move_player_to_next_place(course: &[Space], next_place: usize) -> usize {
    match course.get(next_place) {
        Some(Space::Obstacle(v)) => {
            println!("{OBSTACLE_DISPLAY} space is obstacle. go back {v} spaces");
            hit_obstacle_next_place(next_place, *v)
        }
        Some(Space::FreeSpace) => {
            println!("✅ space is not obstacle.");
            next_place
        }
        None => next_place,
    }
}
