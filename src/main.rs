// Moon Lengths:
//  Short   90
//  Normal 120
//  Long   200

use stacklands_combat_simulator::*;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let num = args.next().and_then(|a| a.parse().ok()).unwrap_or(10);
    let vils = vec![CombatantStats::ninja(); num];

    let stats = simulate(
        1_000,
        &vils,
        &[CombatantStats::demon_lord(); 1],
        0,
        1_000_000,
    );
    println!("{stats}");
}
