// Moon Lengths:
//  Short   90
//  Normal 120
//  Long   200

use stacklands_combat_simulator::*;

fn main() {
    for start in [15, 30, 45, 60, 75] {
        let stats = simulate(
            10_000,
            &[CombatantStats::swordsmen(); 8],
            &[CombatantStats::demon_lord(); 1],
            start,
            90,
        );
        println!("{start} {stats}");
    }
}
