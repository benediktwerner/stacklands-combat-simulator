// Moon Lengths:
//  Short   90
//  Normal 120
//  Long   200

use stacklands_combat_simulator::*;

fn main() {
    let stats = simulate(
        1_000,
        &[CombatantStats::ninja(); 1],
        &[CombatantStats::giant_rat(); 2],
        0,
        1_000_000,
    );
    println!("{stats}");
}
