use std::vec;

use rand::Rng;

fn main() {
    const X_SIZE: usize = 8;
    const Y_SIZE: usize = 8;

    let max_rounds = X_SIZE * Y_SIZE / 2;
    let mut matchfield: [[usize; 8]; 8] = [[0; 8]; 8];

    println!("Max rounds: {}", max_rounds);

    for _ in 1..=max_rounds {
        for player in 1..=2 {
            let free_slots: Vec<usize> = get_free_slots(matchfield);
            let chosen_slot = determine_slot(free_slots);
            insert_coin(&mut matchfield, player, chosen_slot);
        }
    }
    print_matchfield(matchfield)
}

// fn determine_winner(matchfield: [[usize; 8]; 8]) {
//     let mut 
// }

fn get_free_slots(matchfield: [[usize; 8]; 8]) -> Vec<usize> {
    let mut free_slots: Vec<usize> = vec![];
    for x in 0..8 {
        if matchfield[x][matchfield[x].len() -1] == 0 {
            free_slots.push(x)
        }
    }
    return free_slots
}

fn determine_slot(free_slots: Vec<usize>) -> usize {
    let i = rand::thread_rng().gen_range(0..free_slots.len());
    return free_slots[i];
}

fn insert_coin(matchfield: &mut[[usize; 8]; 8], player: usize, x: usize) {
    for y in 0..8 {
        if matchfield[x][y] == 0 {
            matchfield[x][y] = player;
            break;
        }
    }
}

fn print_matchfield(matchfield: [[usize; 8]; 8]) {
    for x in 0..8 {
        for y in 0..8 {
            if matchfield[x][y] == 1 {
                print!("x  ")
            } else if matchfield[x][y] == 2 {
                print!("o  ")
            } else {
                print!("_  ")
            }
            
        }
        println!()
    }
}