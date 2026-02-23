use rand::RngExt;
use wasm_bindgen::prelude::wasm_bindgen;

const MAX_TILE: i32 = 3;
const BETTAH_MULT: f32 = 2.7;
const SIZE: i32 = 16;
const BOMBS: i32 = 4;
const LIVES: i32 = 3;

const OPPONENT_HP: i32 = 1000;
const DAMAGE: i32 = 25;

const QTY1: f32 = (MAX_TILE - 1) as f32;
const QTY3: f32 = ((BETTAH_MULT - 1.0) * 100.0).round();
const TARGET: i32 = OPPONENT_HP / DAMAGE;


const BOOM_BLOCKER: i32 = 2;
const GOLDEN_TILES: i32 = 2;
const BIG_HIT_COMBO: f32 = 0.10;
const FINAL_ROUND_FURY: f32 = 2.0;

const SAFE_TILES: i32 = SIZE - BOMBS;

const P_UP: f32 = 0.14 + (0.06_f32).min(QTY3 / 2000.0) + (0.5_f32).min(0.5 * QTY3 / (QTY3 + 1500.0));
const BASE: f32 = 12.0_f32.min(QTY3 / 150.0);

#[wasm_bindgen]
pub fn run_sim(target: i32, num_sims: i32) -> Vec<i32> {
    (1..=SAFE_TILES).into_iter().map(|flip| {
        let mut rng = rand::rng();
        let mut successes = 0;
        
        for _ in 0..num_sims {
            let mut lives = LIVES;
            let mut score = 0;
            let mut golden_tiles = GOLDEN_TILES;
            let mut boom_blockers = BOOM_BLOCKER;

            while lives > 0 {
                let mut alive = true;
                let mut pool = SIZE;
                let mut bombs = BOMBS;
                
                for _ in 0..flip {
                    if golden_tiles > 0 {
                        golden_tiles -= 1;
                        pool -= 1;
                    } else {
                        let random = rng.random::<f32>();
                        if random < (bombs as f32 / pool as f32) {
                            if boom_blockers > 0 {
                                boom_blockers -= 1;
                                bombs -= 1;
                            } else {
                                alive = false;
                                break;
                            }
                        }
                        pool -= 1
                    }
                }

                if alive {
                    let mut round_score: f32 = 0.0;
                    for _f in 0..flip {
                        let random = rng.random::<f32>();
                        let mut val = (1.0 + (BASE + random ).min(QTY1)).floor() as i32;
                        while rng.random::<f32>() <= P_UP {
                            val += 1;
                            if val >= MAX_TILE {
                                break
                            }
                        }
                        round_score += val as f32
                    }

                    round_score *= 1.0 + (BIG_HIT_COMBO * flip as f32);
                    if lives == 1 {
                        round_score *= FINAL_ROUND_FURY
                    }
                    score += round_score as i32;
                    if score >= target {
                        successes += 1;
                        break;
                    }
                } else {
                    lives -= 1;
                }

            }

        }
        successes
    }).collect()
}
