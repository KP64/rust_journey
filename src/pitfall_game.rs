use rand::Rng;
use std::{cmp::Ordering, io, ops::Range};

const OBSTACLE_DISPLAY: &str = "🔥";
const FREE_SPACE_DISPLAY: &str = "_";

const COURSE_LEN: usize = 32;

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

        let roll = roll_two_dice(1..7, 1..7);
        let roll = roll.0 + roll.1;

        let turn = PlayerTurn {
            roll,
            current_place: place,
        };

        println!("⏭ {:?}", turn);

        let next_place = match find_next_place(&turn, COURSE_LEN, first_roll) {
            NextPlace::GameWon => {
                println!("🏆 Finished the course!");
                break;
            }
            NextPlace::Place(p) => p,
        };

        place = move_player_to_next_place(&course, next_place);

        let snapshot = make_board_display(place, &course);
        println!("🎯 {:?}", snapshot);
        first_roll = false;
        println!("======");
    }
}

fn make_board_display(place: usize, course: &[Space]) -> Vec<String> {
    let mut board = vec![];
    for (i, &spot) in course.iter().enumerate() {
        let item = match spot {
            Space::Obstacle(_) => OBSTACLE_DISPLAY.to_string(),
            Space::FreeSpace => FREE_SPACE_DISPLAY.to_string(),
        };
        if i == place {
            board.push(format!("+{}", i));
        } else {
            board.push(item);
        }
    }
    board
}

fn make_course(len: usize) -> Vec<Space> {
    let mut course = vec![];
    let mut rand_thread = rand::thread_rng();
    for _ in 0..len {
        let space = rand_thread.gen_range(0..=1);

        if space == 0 {
            course.push(Space::FreeSpace);
        } else {
            let penalty = rand_thread.gen_range(2..=3);
            course.push(Space::Obstacle(penalty));
        }
    }
    course
}

fn hit_obstacle_next_place(place: usize, penalty: usize) -> usize {
    let tmp = place - penalty;
    match tmp.cmp(&0) {
        Ordering::Equal | Ordering::Greater => tmp,
        Ordering::Less => 0,
    }
}

///
/// Blocks for user input and rolls the die.
///
fn roll_two_dice(dice1: Range<usize>, dice2: Range<usize>) -> (usize, usize) {
    println!("🎲🎲 Roll two dice…");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed to read line");

    let mut rand_thread = rand::thread_rng();

    (rand_thread.gen_range(dice1), rand_thread.gen_range(dice2))
}

#[derive(Debug)]
struct PlayerTurn {
    roll: usize,
    current_place: usize,
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

fn find_next_place(turn: &PlayerTurn, course_len: usize, first_roll: bool) -> NextPlace {
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
    let space = course.get(next_place);

    match space {
        Some(Space::Obstacle(v)) => {
            println!(
                "{} space is obstacle. go back {} spaces",
                OBSTACLE_DISPLAY, v
            );
            hit_obstacle_next_place(next_place, *v)
        }
        Some(Space::FreeSpace) => {
            println!("✅ space is not obstacle.");
            next_place
        }
        None => next_place,
    }
}
