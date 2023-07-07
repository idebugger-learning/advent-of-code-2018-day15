use crate::map::UnitType;

mod engine;
mod map;

pub const INITIAL_GOBLIN_HP: u16 = 200;
pub const INITIAL_ELF_HP: u16 = 200;

fn main() {
    let input = include_str!("../inputs/input.txt");

    let mut elf_attack_buff = 0;
    loop {
        let (_, mut map) = map::parse_map(input).unwrap();
        let mut round = 0;
        let won = loop {
            println!();
            println!("Round {} started!", round);
            println!("{}", map);

            if let Some(unit_type) = engine::run_round(&mut map, elf_attack_buff) {
                break unit_type;
            }

            round += 1;
        };

        if won == UnitType::Goblin {
            println!("Elves lost, increasing elf attack power");
            elf_attack_buff += 1;
        } else {
            println!();
            println!("Combat ended after {} rounds", round);
            println!("Final map:");
            println!("{}", map);
            println!();
            println!("Winners: {:?}", won);
            println!("Elves attack buff: {}", elf_attack_buff);
            println!("Outcome: {} * {} = {}", round, map.total_hp(), round * map.total_hp());
            break;
        };
    }
}
