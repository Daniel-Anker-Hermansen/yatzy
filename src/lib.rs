use std::iter::repeat;

pub const NUMBER_OF_DIE: usize = 5;
pub const NUMBER_OF_ACTIONS: usize = ACTIONS.len();
pub const NUMBER_OF_POSSIBLE_THROWS: usize = number_of_possible_throws(NUMBER_OF_DIE, 6);
pub const BONUS_REQUIREMENT: usize = 63;

const fn number_of_possible_throws(remaining: usize, max: u8) -> usize {
    if remaining == 0 {
        1
    } else {
        let mut throw = 1;
        let mut acc = 0;
        while throw <= max {
            acc += number_of_possible_throws(remaining - 1, throw);
            throw += 1;
        }
        acc
    }
}

pub struct Action {
    number: fn(&[u8]) -> f64,
    bonus: fn(&[u8]) -> usize,
    pub name: &'static str,
}

impl Action {
    pub fn number(&self, cast: &[u8]) -> f64 {
        (self.number)(cast)
    }

    pub fn bonus(&self, cast: &[u8]) -> usize {
	(self.bonus)(cast)
    }
}

pub const ACTIONS: &[Action] = &[
    Action {
        number: number::<1>,
        name: "ener",
	bonus: bonus::<1>,
    },
    Action {
        number: number::<2>,
        name: "toer",
	bonus: bonus::<2>,
    },
    Action {
        number: number::<3>,
        name: "treer",
	bonus: bonus::<3>,
    },
    Action {
        number: number::<4>,
        name: "fier",
	bonus: bonus::<4>,
    },
    Action {
        number: number::<5>,
        name: "femer",
	bonus: bonus::<5>,
    },
    Action {
        number: number::<6>,
        name: "sekser",
	bonus: bonus::<6>,
    },
    Action {
        number: pair,
        name: "par",
	bonus: |_| 0,
    },
    Action {
        number: two_pair,
        name: "to par",
	bonus: |_| 0,
    },
    Action {
        number: triple,
        name: "tre ens",
	bonus: |_| 0,
    },
    Action {
        number: quad,
        name: "fire ens",
	bonus: |_| 0,
    },
    Action {
        number: full_house,
        name: "fuldt hus",
	bonus: |_| 0,
    },
    Action {
        number: low,
        name: "lav",
	bonus: |_| 0,
    },
    Action {
        number: high,
        name: "høj",
	bonus: |_| 0,
    },
    Action {
        number: chance,
        name: "chance",
	bonus: |_| 0,
    },
    Action {
        number: yatzy,
        name: "yatzy",
	bonus: |_| 0,
    },
];


pub fn number<const N: u8>(cast: &[u8]) -> f64 {
    cast.iter().filter(|c| **c == N).count() as f64 * N as f64
}

fn bonus<const N: u8>(cast: &[u8]) -> usize {
	cast.iter().filter(|k| **k == N).sum::<u8>() as usize
}

fn pair(cast: &[u8]) -> f64 {
    cast.chunk_by(|a, b| a == b)
        .filter(|c| c.len() >= 2)
        .map(|c| c[0] * 2)
        .max()
        .unwrap_or(0) as f64
}

fn triple(cast: &[u8]) -> f64 {
    cast.chunk_by(|a, b| a == b)
        .filter(|c| c.len() >= 3)
        .map(|c| c[0] * 3)
        .max()
        .unwrap_or(0) as f64
}

fn quad(cast: &[u8]) -> f64 {
    cast.chunk_by(|a, b| a == b)
        .filter(|c| c.len() >= 4)
        .map(|c| c[0] * 4)
        .max()
        .unwrap_or(0) as f64
}

fn two_pair(cast: &[u8]) -> f64 {
    let mut v: Vec<u8> = cast
        .chunk_by(|a, b| a == b)
        .filter(|c| c.len() >= 2)
        .map(|c| c[0] * 2)
        .collect();
    v.sort();
    v.reverse();
    if v.len() >= 2 {
        (v[0] + v[1]) as _
    } else {
        0.0
    }
}

fn full_house(cast: &[u8]) -> f64 {
    let pair = cast.chunk_by(|a, b| a == b).filter(|v| v.len() == 2).next();
    let triple = cast.chunk_by(|a, b| a == b).filter(|v| v.len() == 3).next();
    if let (Some(a), Some(b)) = (pair, triple) {
        (2 * a[0] + 3 * b[0]) as f64
    } else {
        0.0
    }
}

fn low(cast: &[u8]) -> f64 {
    if cast == &[1, 2, 3, 4, 5] {
        15.0
    } else {
        0.0
    }
}

fn high(cast: &[u8]) -> f64 {
    if cast == &[2, 3, 4, 5, 6] {
        20.0
    } else {
        0.0
    }
}

fn chance(cast: &[u8]) -> f64 {
    cast.iter().sum::<u8>() as f64
}

fn yatzy(cast: &[u8]) -> f64 {
    if cast.iter().all(|c| *c == cast[0]) {
        50.0
    } else {
        0.0
    }
}

pub fn generate_casts(len: usize) -> Vec<(Vec<u8>, f64)> {
    let mut casts = vec![vec![]];
    for _ in 0..len {
        casts = casts
            .into_iter()
            .flat_map(|v| {
                repeat(v).zip(1..=6).map(|(mut v, j)| {
                    v.push(j);
                    v
                })
            })
            .collect();
    }
    let mut casts: Vec<_> = casts
        .into_iter()
        .zip(repeat(1.0 / 6.0f64.powi(len as _)))
        .collect();
    for d in &mut casts {
        d.0.sort();
    }
    casts.sort_by(|a, b| a.0.cmp(&b.0));
    casts.dedup_by(|a, b| {
        if a.0 == b.0 {
            b.1 += a.1;
            true
        } else {
            false
        }
    });
    casts
}
