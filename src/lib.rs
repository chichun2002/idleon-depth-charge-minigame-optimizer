use rand::RngExt;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run_sim(
    num_sims: i32, 
    boss_health: i32, 
    base_damage: i32, 
    _lives: i32, 
    _bombs: i32,
    max_tile: i32,
    bettah_mult: f32,
    cards: i32,
    _boom_blockers: i32,
    _golden_tiles: i32,
    big_hit_combo: f32,
    final_round_fury: f32,
) -> Vec<i32> {
    let target = boss_health / base_damage;
    let qty1: f32 = (max_tile - 1) as f32;
    let qty3: f32 = ((bettah_mult - 1.0) * 100.0).round();
    let base_value: f32 = 12.0_f32.min(qty3 / 150.0);
    let p_up: f32 = 0.14 + (0.06_f32).min(qty3 / 2000.0) + (0.5_f32).min(0.5 * qty3 / (qty3 + 1500.0));
    (1..=(cards-_bombs)).into_iter().map(|flip| {
        let mut rng = rand::rng();
        let mut successes = 0;
        
        for _ in 0..num_sims {
            let mut lives = _lives;
            let mut score = 0;
            let mut golden_tiles = _golden_tiles;
            let mut boom_blockers = _boom_blockers;

            while lives > 0 {
                let mut alive = true;
                let mut pool = cards;
                let mut bombs = _bombs;
                
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
                        let mut val = (1.0 + (base_value + random).min(qty1)).floor() as i32;
                        while rng.random::<f32>() <= p_up {
                            val += 1;
                            if val >= max_tile {
                                break
                            }
                        }
                        round_score += val as f32
                    }

                    round_score *= 1.0 + (big_hit_combo * flip as f32);
                    if lives == 1 {
                        round_score *= final_round_fury
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
