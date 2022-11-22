#![allow(clippy::cast_precision_loss)]

use std::{cell::RefCell, fmt::Display, rc::Rc};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub mod js;

const TIME_STEP: u32 = 5; // time of attack stun
const ATTACK_STUN_TIME: u32 = 5;
const MELEE_ATTACK_WINDUP_TIME: u32 = 50;
const MELEE_ATTACK_WINDDOWN_TIME: u32 = 50;
const MAGIC_WAIT_TIME: u32 = 30;
const PROJECTILE_SPEED: f32 = 4.5;
const MAX_DMG: u32 = 6;
const BATTLE_X_DIFF: f32 = 0.75;
const BATTLE_Y_DIFF_2: f32 = 0.85 * 0.85;

enum UpdateResult {
    Nothing,
    StartedAttack,
    CombatEnded,
}

#[derive(Clone, Copy, Debug, Deserialize, Tsify)]
pub enum AttackType {
    Magic,
    Melee,
    Ranged,
}
impl AttackType {
    #[must_use]
    fn is_effective_vs(self, other: AttackType) -> bool {
        matches!(
            (self, other),
            (AttackType::Magic, AttackType::Ranged)
                | (AttackType::Melee, AttackType::Magic)
                | (AttackType::Ranged, AttackType::Melee)
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Tsify)]
pub struct CombatantStats {
    pub hp: u32,
    pub attack_speed: u32, // in centi-seconds (1 = 0.01s)
    pub hit_chance: f64,   // 0-1
    // pub stun_chance: f64,  // 0-1
    pub attack_type: AttackType,
    pub damage: u32,
    pub defense: u32,
}

#[derive(Debug)]
struct Combatant {
    stats: CombatantStats,
    hp: u32,
    attack_cooldown: u32,
    getting_attacked: bool,
    attacking_timer: u32,
    stunned_timer: u32,
    targets: Vec<Rc<RefCell<Combatant>>>,
    target_index: usize,
    hit: bool,
}

impl CombatantStats {
    #[must_use]
    pub const fn swordsmen() -> Self {
        CombatantStats {
            hp: 15,
            attack_speed: 150,
            hit_chance: 0.8,
            damage: 3,
            defense: 1,
            attack_type: AttackType::Melee,
        }
    }

    #[must_use]
    pub const fn ninja() -> Self {
        CombatantStats {
            hp: 15,
            attack_speed: 75,
            hit_chance: 0.8,
            damage: 3,
            defense: 1,
            attack_type: AttackType::Ranged,
        }
    }

    #[must_use]
    pub const fn demon_lord() -> Self {
        CombatantStats {
            hp: 666,
            attack_speed: 75,
            hit_chance: 0.8,
            damage: 6,
            defense: 4,
            attack_type: AttackType::Melee,
        }
    }

    #[must_use]
    pub const fn giant_rat() -> Self {
        CombatantStats {
            hp: 20,
            attack_speed: 150,
            hit_chance: 0.7,
            damage: 3,
            defense: 1,
            attack_type: AttackType::Melee,
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
            targets: vec![],
            target_index: 0,
            hit: false,
        }
    }

    // true if dead
    #[must_use]
    fn damage(
        &mut self,
        hit: bool,
        attack_damage: u32,
        attack_type: AttackType,
        rng: &mut SmallRng,
    ) -> bool {
        if self.hp == 0 {
            return false;
        }

        self.stunned_timer = ATTACK_STUN_TIME;

        if !hit {
            return false;
        }

        let mut dmg = attack_damage;
        if dmg < MAX_DMG && rng.gen::<bool>() {
            dmg += 1;
        }
        if dmg > self.stats.defense {
            dmg -= self.stats.defense;
            if attack_type.is_effective_vs(self.stats.attack_type) {
                dmg = ((dmg as f32) * 1.4).round() as u32;
            }
        } else if rng.gen::<bool>() {
            dmg = 1;
        } else {
            return false;
        };

        if dmg >= self.hp {
            self.hp = 0;
            return true;
        }

        self.hp -= dmg;

        false
    }

    fn update(
        &mut self,
        can_attack: bool,
        team_index: usize,
        team_size: usize,
        rng: &mut SmallRng,
        enemies: &mut Vec<Rc<RefCell<Combatant>>>,
    ) -> UpdateResult {
        if self.stunned_timer > 0 {
            self.stunned_timer -= TIME_STEP;
        }

        if self.attacking_timer > 0 {
            self.attacking_timer -= TIME_STEP;
            if self.attacking_timer == 0 {
                return self.perform_attack(rng, enemies);
            }
            return UpdateResult::Nothing;
        }

        if !can_attack {
            return UpdateResult::Nothing;
        }

        if self.attack_cooldown > 0 {
            self.attack_cooldown -= TIME_STEP;
            if self.attack_cooldown > 0 {
                return UpdateResult::Nothing;
            }
        }

        self.start_attack(team_index, team_size, rng, enemies)
    }

    fn perform_attack(
        &mut self,
        rng: &mut SmallRng,
        enemies: &mut Vec<Rc<RefCell<Combatant>>>,
    ) -> UpdateResult {
        if let AttackType::Melee = self.stats.attack_type {
            if self.target_index == self.targets.len() {
                for target in &mut self.targets {
                    target.borrow_mut().getting_attacked = false;
                }
            } else {
                let dmg = self.stats.damage;
                let typ = self.stats.attack_type;
                let hit = self.hit;
                let target = &mut self.targets[self.target_index];
                let killed = target.borrow_mut().damage(hit, dmg, typ, rng);
                if killed && kill(enemies, target) {
                    return UpdateResult::CombatEnded;
                }
                self.target_index += 1;
                self.attacking_timer = if self.target_index == self.targets.len() {
                    MELEE_ATTACK_WINDDOWN_TIME
                } else {
                    MELEE_ATTACK_WINDUP_TIME + MELEE_ATTACK_WINDDOWN_TIME
                };
                return UpdateResult::Nothing;
            }
        } else {
            self.stunned_timer = ATTACK_STUN_TIME;

            let dmg = self.stats.damage;
            let typ = self.stats.attack_type;
            let hit = self.hit;
            for target in &mut self.targets {
                let killed = {
                    let mut target = target.borrow_mut();
                    target.getting_attacked = false;
                    target.damage(hit, dmg, typ, rng)
                };
                if killed && kill(enemies, target) {
                    return UpdateResult::CombatEnded;
                }
            }
        }
        self.targets.clear();
        self.attack_cooldown = self.stats.attack_speed;
        UpdateResult::Nothing
    }

    fn start_attack(
        &mut self,
        team_index: usize,
        team_size: usize,
        rng: &mut SmallRng,
        enemies: &mut [Rc<RefCell<Combatant>>],
    ) -> UpdateResult {
        if self.getting_attacked || self.stunned_timer > 0 {
            return UpdateResult::Nothing;
        }

        let (index, target) = find_target(team_index, team_size, rng, enemies);
        target.borrow_mut().getting_attacked = true;
        self.targets.push(target);
        self.target_index = 0;
        self.hit = rng.gen_bool(self.stats.hit_chance);
        self.attacking_timer = match self.stats.attack_type {
            AttackType::Melee => MELEE_ATTACK_WINDUP_TIME,
            AttackType::Magic => {
                MAGIC_WAIT_TIME + projectile_time(team_index, team_size, index, enemies.len())
            }
            AttackType::Ranged => projectile_time(team_index, team_size, index, enemies.len()),
        };

        UpdateResult::StartedAttack
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
        for (i, v) in self.enemy_hp.iter().rev().enumerate().rev() {
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

    let start_time = start_time * 100;
    let month_length = month_length * 100;

    for _ in 0..iters {
        let mut time = start_time;
        let mut last_attack = 0;
        let mut villagers: Vec<Rc<RefCell<Combatant>>> = villager_setup
            .iter()
            .map(|s| Rc::new(RefCell::new(Combatant::new(s))))
            .collect();
        let mut enemies: Vec<Rc<RefCell<Combatant>>> = enemy_setup
            .iter()
            .map(|s| Rc::new(RefCell::new(Combatant::new(s))))
            .collect();

        'outer: loop {
            time += TIME_STEP;
            let mut can_attack = time - last_attack >= 30;

            if time > 100_000 {
                println!("timeout");
                break;
            }

            if time % month_length == 0 {
                for v in &villagers {
                    let mut v = v.borrow_mut();
                    v.hp = (v.hp + 3).min(v.stats.hp);
                }
            }

            for (i, c) in villagers.iter().enumerate() {
                match c
                    .borrow_mut()
                    .update(can_attack, i, villagers.len(), &mut rng, &mut enemies)
                {
                    UpdateResult::StartedAttack => {
                        last_attack = time;
                        can_attack = false;
                    }
                    UpdateResult::CombatEnded => break 'outer,
                    UpdateResult::Nothing => (),
                }
            }

            for (i, c) in enemies.iter().enumerate() {
                match c
                    .borrow_mut()
                    .update(can_attack, i, enemies.len(), &mut rng, &mut villagers)
                {
                    UpdateResult::StartedAttack => {
                        last_attack = time;
                        can_attack = false;
                    }
                    UpdateResult::CombatEnded => break 'outer,
                    UpdateResult::Nothing => (),
                }
            }
        }

        let length = (time - start_time) / 100;
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

fn find_target(
    team_index: usize,
    team_size: usize,
    rng: &mut SmallRng,
    enemies: &[Rc<RefCell<Combatant>>],
) -> (usize, Rc<RefCell<Combatant>>) {
    let num = team_size as f32;
    let num2 = team_index as f32;
    let num3 = enemies.len() as f32 / num;
    let f = num2 * num3;
    let f2 = (num2 + 1.0) * num3;
    let min = f.floor() as usize;
    let max = f2.ceil() as usize;

    let index = rng.gen_range(min..max);
    (index, Rc::clone(&enemies[index]))
}

fn projectile_time(
    team_index: usize,
    team_size: usize,
    enemy_index: usize,
    enemy_size: usize,
) -> u32 {
    let x1 = (team_index as f32) + 0.5 - (team_size as f32) / 2.0;
    let x2 = (enemy_index as f32) + 0.5 - (enemy_size as f32) / 2.0;
    let xd = (x1 - x2) * BATTLE_X_DIFF;
    let dist = (xd * xd + BATTLE_Y_DIFF_2).sqrt();
    let time = dist / PROJECTILE_SPEED * (100.0 / TIME_STEP as f32);
    time.round() as u32 * TIME_STEP
}

fn kill(enemies: &mut Vec<Rc<RefCell<Combatant>>>, target: &mut Rc<RefCell<Combatant>>) -> bool {
    if let Some(pos) = enemies.iter().position(|e| Rc::ptr_eq(e, target)) {
        enemies.remove(pos);
    }
    enemies.is_empty()
}
