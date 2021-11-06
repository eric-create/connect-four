use std::{char, vec};

use rand::Rng;

fn main() {
    const X_SIZE: u8 = 8;
    const Y_SIZE: u8 = 8;

    let max_rounds = X_SIZE * Y_SIZE / 2;
    let mut matchfield: [[u8; 8]; 8] = [[0; 8]; 8];
    let mut coordinate: (usize, usize) = (0, 0);
    let mut points: u8 = 0;

    println!("Max rounds: {}", max_rounds);

    'outer: for _ in 1..=max_rounds {
        for player in 1..=2 {
            let free_slots: Vec<usize> = get_free_slots(matchfield);
            let chosen_slot: usize = determine_slot(free_slots);
            coordinate = insert_coin(&mut matchfield, player, chosen_slot);
            points = determine_points(matchfield, player, coordinate);
            if points >= 4 {
                let player_name: char = if player == 1 { 'x' } else { 'o' };
                println!("Player {} wins at {} {}", player_name, coordinate.0, coordinate.1);
                break 'outer
            }
        }
    }
    print_result(matchfield, coordinate, points)
}

fn determine_points(matchfield: [[u8; 8]; 8], player: u8, coordinate: (usize, usize)) -> u8 {
    fn get_direction_points(matchfield: [[u8; 8]; 8], player: u8, coordinate: (usize, usize), x_direction: isize, y_direction: isize) -> u8 {
        let mut points: u8 = 0;
        let mut x: isize = coordinate.0 as isize;
        let mut y: isize = coordinate.1 as isize;
        let x_limit: isize = if x_direction > 0 { 8 } else { -1 };
        let y_limit: isize = if y_direction > 0 { 8 } else { -1 };
   
        while x + x_direction != x_limit && y + y_direction != y_limit {
            x = x + x_direction;
            y = y + y_direction;
            if matchfield[x as usize][y as usize] == player {
                points += 1;
            } else {
                break
            }
        }
        points
    }

    let horizontal_points: u8 = 
        get_direction_points(matchfield, player, coordinate, -1, 0)
        + get_direction_points(matchfield, player, coordinate, 1, 0);

    let vertical_points: u8 = get_direction_points(matchfield, player, coordinate, 0, -1);
    let mut max_points: u8 = if vertical_points > horizontal_points { vertical_points } else { horizontal_points };

    let upward_points: u8 = 
        get_direction_points(matchfield, player, coordinate, 1, 1)
        + get_direction_points(matchfield, player, coordinate, -1, -1);
    max_points = if upward_points > max_points { upward_points } else { max_points };

    let donward_points: u8 = 
        get_direction_points(matchfield, player, coordinate, 1, -1)
        + get_direction_points(matchfield, player, coordinate, -1, 1);
    max_points = if donward_points > max_points { donward_points } else { max_points };

    max_points + 1
}


fn get_free_slots(matchfield: [[u8; 8]; 8]) -> Vec<usize> {
    let mut free_slots: Vec<usize> = vec![];
    for x in 0..8 {
        if matchfield[x][matchfield[x].len() -1] == 0 {
            free_slots.push(x)
        }
    }
    return free_slots
}

fn determine_slot(free_slots: Vec<usize>) -> usize {
    let i: usize = rand::thread_rng().gen_range(0..free_slots.len());
    return free_slots[i];
}

fn insert_coin(matchfield: &mut[[u8; 8]; 8], player: u8, x: usize) -> (usize, usize) {
    let mut y: usize = 0;

    for i in 0..8 {
        y = i;
        if matchfield[x][i] == 0 {
            matchfield[x][i] = player;
            break;
        }
    }
    (x, y)
}

fn print_result(matchfield: [[u8; 8]; 8], winning_coordinate: (usize, usize), points: u8) {
    for y in (0..8).rev() {
        for x in (0..8).rev() {
            fn print(name: char, x: usize, y: usize, winning_coordinate: (usize, usize), points: u8) {
                if points >= 4 && x == winning_coordinate.0 && y == winning_coordinate.1 {
                    print!("W ")
                } else {
                    print!("{} ", name)
                }
            }

            if matchfield[x][y] == 1 {
                print('x', x, y, winning_coordinate, points);
            } else if matchfield[x][y] == 2 {
                print('o', x, y, winning_coordinate, points);
            } else {
                print(' ', x, y, winning_coordinate, points);
            }
            
        }
        println!()
    }
    if points >= 4 {
        let winner = if matchfield[winning_coordinate.0][winning_coordinate.1] == 1 { 'x' } else { 'o' };
        println!("\nAnd the winner is {}\n", winner)
    } else {
        println!("\nNobody wins\n")
    }
}