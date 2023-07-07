mod engine;
mod map;

pub const INITIAL_GOBLIN_HP: u16 = 200;
pub const INITIAL_ELF_HP: u16 = 200;

fn main() {
    let input = include_str!("../inputs/input.txt");
    let (_, mut map) = map::parse_map(input).unwrap();

    let mut round = 0;
    loop {
        println!();
        println!("Round {} started!", round);
        println!("{}", map);

        if !engine::run_round(&mut map, 0) {
            break;
        }

        round += 1;
    }

    println!();
    println!("Combat ended after {} rounds", round);
    println!("Final map:");
    println!("{}", map);
    println!();
    println!("Outcome: {} * {} = {}", round, map.total_hp(), round * map.total_hp());
}
