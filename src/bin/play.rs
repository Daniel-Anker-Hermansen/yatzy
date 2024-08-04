use std::{collections::HashMap, io::Read};

use yatzy::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let casts = generate_casts(NUMBER_OF_DIE);
    assert_eq!(casts.len(), NUMBER_OF_POSSIBLE_THROWS);

    let reverse_map: HashMap<&[u8], usize> =
        casts.iter().map(|(v, _)| v.as_slice()).zip(0..).collect();
    
    let mut dp = vec![[[0u8; NUMBER_OF_POSSIBLE_THROWS]; 3]; 1 << NUMBER_OF_ACTIONS];
    
    let len = 3 * NUMBER_OF_POSSIBLE_THROWS * (1 << NUMBER_OF_ACTIONS);

    let slice = unsafe { std::slice::from_raw_parts_mut(dp.as_mut_ptr().cast(), len) };

    std::fs::File::open(path).unwrap().read_exact(slice).unwrap();

    let mut s = String::new();
    let mut finished = 0;
    let mut score = 0.0;
    for _ in 0..NUMBER_OF_ACTIONS {
        println!("Enter the results:");

        std::io::stdin().read_line(&mut s).unwrap();
        let mut cast: Vec<u8> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
        s.clear();
        cast.sort();

        let cast_num = reverse_map[cast.as_slice()];
        let action = dp[finished][0][cast_num];
        print!("Reroll");
        for i in 0..NUMBER_OF_DIE {
            if action & (1 << i) == 0 {
                print!(" {}", cast[i]);
            }
        }
        println!();

        println!("Enter the results (all die):");

        std::io::stdin().read_line(&mut s).unwrap();
        let mut cast: Vec<u8> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
        s.clear();
        cast.sort();

        let cast_num = reverse_map[cast.as_slice()];
        let action = dp[finished][1][cast_num];
        print!("Reroll");
        for i in 0..NUMBER_OF_DIE {
            if action & (1 << i) == 0 {
                print!(" {}", cast[i]);
            }
        }
        println!();

        println!("Enter the results (all die):");

        std::io::stdin().read_line(&mut s).unwrap();
        let mut cast: Vec<u8> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
        s.clear();
        cast.sort();

        let cast_num = reverse_map[cast.as_slice()];
        let action = dp[finished][2][cast_num];

        finished |= 1 << action;
        score += ACTIONS[action as usize].number(cast.as_slice());
        println!(
            "Enact action {}. Score is now {}",
            ACTIONS[action as usize].name, score
        );
    }

    println!("Score was: {}", score);
}
