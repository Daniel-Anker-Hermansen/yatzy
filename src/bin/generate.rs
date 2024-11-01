use std::{collections::HashMap, io::Write};

use yatzy::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let casts = generate_casts(NUMBER_OF_DIE);
    assert_eq!(casts.len(), NUMBER_OF_POSSIBLE_THROWS);

    let reverse_map: HashMap<&[u8], usize> =
        casts.iter().map(|(v, _)| v.as_slice()).zip(0..).collect();

    let choice_distributions: Vec<Vec<(u8, Vec<(usize, f64)>)>> = casts
        .iter()
        .map(|(d, _)| {
            let mut chosen: Vec<(u8, Vec<_>)> = (0..1 << NUMBER_OF_DIE)
                .map(|bitset| {
                    (
                        bitset,
                        (0..NUMBER_OF_DIE)
                            .filter(|i| (bitset >> i) & 1 == 1)
                            .collect(),
                    )
                })
                .collect();
            chosen.sort_by(|(_, a), (_, b)| a.iter().map(|i| d[*i]).cmp(b.iter().map(|i| d[*i])));
            chosen.dedup_by(|(_, a), (_, b)| a.iter().map(|i| d[*i]).eq(b.iter().map(|i| d[*i])));

            chosen
                .into_iter()
                .map(|(bitset, choice)| {
                    let cases = generate_casts(NUMBER_OF_DIE - choice.len());

                    let distribution = cases
                        .into_iter()
                        .map(|(case, probability)| {
                            let mut cast: Vec<u8> =
                                choice.iter().map(|i| d[*i]).chain(case).collect();
                            cast.sort();
                            let index = reverse_map[cast.as_slice()];
                            (index, probability)
                        })
                        .collect();
                    (bitset, distribution)
                })
                .collect()
        })
        .collect();

    let mut dp = vec![
        [[[0u8; NUMBER_OF_POSSIBLE_THROWS]; 3]; 1 << NUMBER_OF_ACTIONS];
        BONUS_REQUIREMENT + 1
    ];
    let mut scratch = vec![[0.0f64; NUMBER_OF_POSSIBLE_THROWS]; 3];
    let mut values = vec![[0.0f64; 1 << NUMBER_OF_ACTIONS]; BONUS_REQUIREMENT + 1];

    // Set the value to 50 for having no bonus requirement left and having done all actions.
    values[0][(1 << NUMBER_OF_ACTIONS) - 1] = 50.0;
    for bonus in 0..=BONUS_REQUIREMENT {
	eprintln!("Bonus: {}", bonus);
        for finished in (0..1 << NUMBER_OF_ACTIONS).rev().skip(1) {
            for i in 0..3 {
                for j in 0..NUMBER_OF_POSSIBLE_THROWS {
                    scratch[i][j] = 0.0;
                }
            }
            for cast in 0..NUMBER_OF_POSSIBLE_THROWS {
                for cat in 0..NUMBER_OF_ACTIONS {
                    if finished & (1 << cat) == 0 {
                        let acquired_bonus = ACTIONS[cat].bonus(&casts[cast].0);
                        let value = ACTIONS[cat].number(&casts[cast].0)
                            + values[bonus.saturating_sub(acquired_bonus)][finished | (1 << cat)];
                        if value >= scratch[2][cast] {
                            scratch[2][cast] = value;
                            dp[bonus][finished][2][cast] = cat as u8;
                        }
                    }
                }
            }
            for i in (0..2).rev() {
                for cast in 0..NUMBER_OF_POSSIBLE_THROWS {
                    for (bitset, choice_distribution) in &choice_distributions[cast] {
                        let value = choice_distribution
                            .iter()
                            .map(|(j, b)| scratch[i + 1][*j] * b)
                            .sum();
                        if value >= scratch[i][cast] {
                            scratch[i][cast] = value;
                            dp[bonus][finished][i][cast] = *bitset;
                        }
                    }
                }
            }
            values[bonus][finished] = scratch[0]
                .iter()
                .zip(&casts)
                .map(|(a, (_, b))| a * b)
                .sum::<f64>();
        }
    }

    let len = 3 * NUMBER_OF_POSSIBLE_THROWS * (1 << NUMBER_OF_ACTIONS) * (BONUS_REQUIREMENT + 1);

    let slice = unsafe { std::slice::from_raw_parts(dp.as_ptr().cast(), len) };

    std::fs::File::create(path)
        .unwrap()
        .write_all(slice)
        .unwrap();
}
