// Swordsmen
// Attack Speed: 2.5s
// Hit Chance: 90%
// Damage: 2
// HP: 7

// Demon Lord
// Attack Speed: 1.5s
// Hit Chance: 75%
// Damage: 1-3
// HP: 666

// Moon Lengths:
//  Short   90
//  Normal 120
//  Long   200

use rand::{rngs::SmallRng, Rng, SeedableRng};

fn main() {
    const ITERS: u32 = 1_000_000;
    let mut rng = SmallRng::from_entropy();

    for vills in 5..=20_usize {
        // let vills: usize = 8;

        for start_time in [0, 15, 30, 45] {
            let mut wins: u32 = 0;
            let mut demon_hp_left: u64 = 0;
            let mut alive: u32 = 0;

            for _ in 0..ITERS {
                let mut villagers = vec![7_u32; vills];
                let mut demon_hp: u32 = 666;

                for time in start_time.. {
                    if time % 60 == 0 {
                        villagers.iter_mut().for_each(|v| *v = (*v + 3).min(7));
                    }

                    if vill_attack(&mut rng, villagers.len(), &mut demon_hp) {
                        break;
                    }

                    if demon_attack(&mut rng, &mut villagers) {
                        break;
                    }
                }

                if villagers.is_empty() {
                    demon_hp_left += demon_hp as u64;
                } else {
                    wins += 1;
                    alive += villagers.len() as u32;
                }
            }

            println!(
                "{vills:>2} {start_time:>2}: {:>6.2}% wins   {:>4.1} alive   {:>3} HP left",
                (wins as f64 / ITERS as f64) * 100.0,
                if wins > 0 {
                    alive as f64 / wins as f64
                } else {
                    0.0
                },
                if wins < ITERS {
                    demon_hp_left / (ITERS - wins) as u64
                } else {
                    0
                }
            );
        }
    }
}

#[inline(always)]
fn demon_attack(rng: &mut SmallRng, villagers: &mut Vec<u32>) -> bool {
    if rng.gen_bool(0.75) {
        let target = rng.gen_range(0..villagers.len());
        let dmg = rng.gen_range(1..4);
        unsafe {
            if *villagers.get_unchecked(target) <= dmg {
                villagers.swap_remove(target);
                if villagers.is_empty() {
                    return true;
                }
            } else {
                *villagers.get_unchecked_mut(target) -= dmg;
            }
        }
    }
    false
}

#[inline(always)]
fn vill_attack(rng: &mut SmallRng, vills: usize, demon_hp: &mut u32) -> bool {
    for _ in 0..vills {
        if rng.gen_bool(0.9) {
            if *demon_hp <= 3 {
                return true;
            }
            *demon_hp -= 2;
        }
    }
    false
}
