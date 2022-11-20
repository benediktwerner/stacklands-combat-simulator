use std::{cell::RefCell, fmt::Display, rc::Rc};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub mod js;

const STUN_TIME: u32 = 51; // centi-seconds
const ATTACK_TIME: u32 = 2; // centi-seconds

#[derive(Clone, Copy, Debug, Deserialize, Tsify)]
pub struct CombatantStats {
    pub hp: u32,
    pub attack_speed: u32, // in centi-seconds
    pub hit_chance: f64,   // 0-1
    pub stun_chance: f64,  // 0-1
    pub min_damage: u32,
    pub max_damage: u32,
}

#[derive(Debug)]
struct Combatant {
    stats: CombatantStats,
    hp: u32,
    attack_cooldown: u32,
    getting_attacked: bool,
    attacking_timer: u32,
    stunned_timer: u32,
    target: Option<Rc<RefCell<Combatant>>>,
}

impl CombatantStats {
    fn gen_dmg(&self, rng: &mut SmallRng) -> u32 {
        if self.min_damage == self.max_damage {
            self.min_damage
        } else {
            rng.gen_range(self.min_damage..=self.max_damage)
        }
    }

    #[must_use]
    pub const fn swordsmen() -> Self {
        CombatantStats {
            hp: 7,
            attack_speed: 15,
            hit_chance: 0.9,
            stun_chance: 0.0,
            min_damage: 2,
            max_damage: 2,
        }
    }

    #[must_use]
    pub const fn demon_lord() -> Self {
        CombatantStats {
            hp: 666,
            attack_speed: 15,
            hit_chance: 0.75,
            stun_chance: 0.0,
            min_damage: 1,
            max_damage: 3,
        }
    }
}

impl Combatant {
    const fn new(stats: &CombatantStats) -> Self {
        Self {
            stats: *stats,
            hp: stats.hp,
            attack_cooldown: stats.attack_speed,
            getting_attacked: false,
            attacking_timer: 0,
            stunned_timer: 0,
            target: None,
        }
    }

    fn update(&mut self, rng: &mut SmallRng, enemies: &mut Vec<Rc<RefCell<Combatant>>>) -> bool {
        if self.stunned_timer > 0 {
            self.stunned_timer -= 1;
        }

        if self.attack_cooldown > 0 {
            self.attack_cooldown -= 1;
            if self.attack_cooldown > 0 {
                return false;
            }
        } else if self.attacking_timer > 0 {
            self.attacking_timer -= 1;
            if self.attacking_timer == 0 {
                if let Some(target) = self.target.as_deref() {
                    target.borrow_mut().getting_attacked = false;
                    self.target = None;
                }
                self.attack_cooldown = self.stats.attack_speed;
            }
            return false;
        }

        self.attack(rng, enemies)
    }

    fn attack(&mut self, rng: &mut SmallRng, enemies: &mut Vec<Rc<RefCell<Combatant>>>) -> bool {
        if self.getting_attacked || self.stunned_timer > 0 {
            return false;
        }

        let target_index = match find_target(rng, enemies) {
            Some(target) => target,
            None => return false,
        };

        let target = unsafe { enemies.get_unchecked(target_index) };
        self.attacking_timer = ATTACK_TIME;

        if rng.gen_bool(self.stats.hit_chance) {
            let dmg = self.stats.gen_dmg(rng);
            if target.borrow().hp <= dmg {
                let target = enemies.swap_remove(target_index);
                if let Some(t) = target.borrow_mut().target.as_deref() {
                    t.borrow_mut().getting_attacked = false;
                }
                return enemies.is_empty();
            }
            self.target = Some(Rc::clone(target));
            let mut target = target.borrow_mut();
            target.getting_attacked = true;
            target.hp -= dmg;
            if self.stats.stun_chance > 0.0 && rng.gen_bool(self.stats.stun_chance) {
                target.stunned_timer = STUN_TIME;
            }
            false
        } else {
            self.target = Some(Rc::clone(target));
            target.borrow_mut().getting_attacked = true;
            false
        }
    }
}

#[derive(Default, Serialize, Tsify)]
pub struct Stats {
    pub iters: u32,
    pub wins: u32,
    pub timeouts: u32,
    pub total_length: u64,
    pub longest: u32,
    pub village_survivors: Vec<u32>,
    pub enemy_hp: [u32; 10],
}

impl Stats {
    fn new(iters: u32, villagers: usize) -> Self {
        Self {
            iters,
            wins: 0,
            timeouts: 0,
            total_length: 0,
            longest: 0,
            village_survivors: vec![0; villagers],
            enemy_hp: [0; 10],
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{:>6.2}% {:>4}s",
            (100 * self.wins) as f64 / self.iters as f64,
            self.total_length / self.iters as u64
        )?;
        let max_count = self
            .village_survivors
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
            .max(self.enemy_hp.iter().copied().max().unwrap_or(0));
        for (i, v) in self.enemy_hp.iter().enumerate().rev() {
            let width = (v * 300 / max_count) as usize;
            writeln!(f, "{i} {:#<width$}", "")?;
        }
        writeln!(f, "---")?;
        for (i, v) in self.village_survivors.iter().enumerate() {
            let width = (v * 300 / max_count) as usize;
            writeln!(f, "{} {:#<width$}", i + 1, "")?;
        }
        Ok(())
    }
}

#[must_use]
pub fn simulate(
    iters: u32,
    villager_setup: &[CombatantStats],
    enemy_setup: &[CombatantStats],
    start_time: u32,
    month_length: u32,
) -> Stats {
    console_error_panic_hook::set_once();

    let mut rng = SmallRng::from_entropy();
    let mut stats = Stats::new(iters, villager_setup.len());
    let total_enemy_hp: usize = enemy_setup.iter().map(|e| e.hp).sum::<u32>() as usize;

    let start_time = start_time * 10;
    let month_length = month_length * 10;

    for _ in 0..iters {
        let mut time = start_time;
        let mut villagers: Vec<Rc<RefCell<Combatant>>> = villager_setup
            .iter()
            .map(|s| Rc::new(RefCell::new(Combatant::new(s))))
            .collect();
        let mut enemies: Vec<Rc<RefCell<Combatant>>> = enemy_setup
            .iter()
            .map(|s| Rc::new(RefCell::new(Combatant::new(s))))
            .collect();

        'outer: loop {
            time += 1;
            if time > 100_000 {
                break;
            }

            if time % month_length == 0 {
                for v in &villagers {
                    let mut v = v.borrow_mut();
                    v.hp = (v.hp + 3).min(v.stats.hp);
                }
            }

            for c in &villagers {
                if c.borrow_mut().update(&mut rng, &mut enemies) {
                    break 'outer;
                }
            }

            for c in &enemies {
                if c.borrow_mut().update(&mut rng, &mut villagers) {
                    break 'outer;
                }
            }

            for c in &villagers {
                let mut v = c.borrow_mut();
                if v.attack_cooldown == 0
                    && v.attacking_timer == 0
                    && v.attack(&mut rng, &mut enemies)
                {
                    break 'outer;
                }
            }
        }

        let length = (time - start_time) / 10;
        stats.total_length += length as u64;
        stats.longest = stats.longest.max(length);
        if enemies.is_empty() {
            stats.wins += 1;
            stats.village_survivors[villagers.len() - 1] += 1;
        } else if villagers.is_empty() {
            let hp = enemies.iter().map(|e| e.borrow().hp).sum::<u32>() as usize - 1;
            stats.enemy_hp[9 - hp * 10 / total_enemy_hp] += 1;
        } else {
            stats.timeouts += 1;
        }
    }

    stats
}

fn find_target(rng: &mut SmallRng, enemies: &[Rc<RefCell<Combatant>>]) -> Option<usize> {
    let mut targets = 0;
    for e in enemies {
        if !e.borrow().getting_attacked {
            targets += 1;
        }
    }
    let mut target = match targets {
        0 => return None,
        1 => 0,
        _ => rng.gen_range(0..targets),
    };
    for (i, e) in enemies.iter().enumerate() {
        if !e.borrow().getting_attacked {
            if target == 0 {
                return Some(i);
            }
            target -= 1;
        }
    }
    unreachable!("there should be a target");
}
