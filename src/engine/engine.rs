use std::cmp::Ordering;
use crate::map::{Map, MapElement, Position, Terrain, Unit, UnitType};

enum UnitAction {
    Move(Position),
    Attack(Position, u16),
    None,
}

struct Target {
    unit: Unit,
    position: Position,
    distance: usize,
}

pub fn run_round(map: &mut Map, elves_buff: u16) -> Option<UnitType> {
    let units = map.get_units_positions();
    for unit in units {
        if !are_enemies_left(map) {
            let units = map.get_units_positions();
            return Some(units[0].details.0);
        }

        if map.get_unit(unit.position).is_none() {
            continue;
        }

        let elves_count = map
            .get_units_positions()
            .iter()
            .filter(|unit| unit.details.0 == UnitType::Elf)
            .count();

        let first_action = run_unit_strategy(map, unit, elves_buff);
        let is_action_move = matches!(first_action, UnitAction::Move(_));
        let new_unit_position = do_unit_action(map, unit, first_action);

        if is_action_move {
            let unit = Unit {
                details: unit.details,
                position: new_unit_position,
            };
            let second_action = run_unit_strategy(map, unit, elves_buff);
            if matches!(second_action, UnitAction::Attack(_, _)) {
                do_unit_action(map, unit, second_action);
            }
        }

        let new_elves_count = map
            .get_units_positions()
            .iter()
            .filter(|unit| unit.details.0 == UnitType::Elf)
            .count();
        if new_elves_count < elves_count {
            return Some(UnitType::Goblin);
        }
    }

    return None;
}

fn are_enemies_left(map: &Map) -> bool {
    let units = map.get_units_positions();
    let goblins_left = units
        .iter()
        .any(|unit| unit.details.0 == UnitType::Goblin);
    let elves_left = units.iter().any(|unit| unit.details.0 == UnitType::Elf);
    return goblins_left && elves_left;
}

fn run_unit_strategy(map: &Map, unit: Unit, elves_buff: u16) -> UnitAction {
    let enemy_type = unit.details.0.get_enemy_type();
    let possible_targets = map
        .get_units_positions()
        .into_iter()
        .filter(|unit| unit.details.0 == enemy_type)
        .collect::<Vec<_>>();
    if possible_targets.is_empty() {
        return UnitAction::None;
    }

    let mut reachable_targets = find_reachable_targets(map, unit, &possible_targets);
    if reachable_targets.is_empty() {
        return UnitAction::None;
    }

    reachable_targets.sort_by(|a, b| {
        if a.distance == b.distance {
            if a.distance == 0 {
                if a.unit.details.1 == b.unit.details.1 {
                    if a.position.1.cmp(&b.position.1) == Ordering::Equal {
                        return a.position.0.cmp(&b.position.0);
                    } else {
                        return a.position.1.cmp(&b.position.1);
                    }
                }
                return a.unit.details.1.cmp(&b.unit.details.1);
            }
            if a.position.1.cmp(&b.position.1) == Ordering::Equal {
                return a.position.0.cmp(&b.position.0);
            } else {
                return a.position.1.cmp(&b.position.1);
            }
        }
        return a.distance.cmp(&b.distance);
    });
    let closest_target = match reachable_targets.first() {
        Some(target) => target,
        None => unreachable!("We have checked that there is at least one target"),
    };

    if closest_target.distance == 0 {
        let buff = if unit.details.0 == UnitType::Elf {
            elves_buff
        } else {
            0
        };
        return UnitAction::Attack(closest_target.unit.position, buff);
    }

    return UnitAction::Move(closest_target.position);
}

fn find_reachable_targets<'a>(
    map: &'_ Map,
    unit: Unit,
    possible_targets: &'a [Unit],
) -> Vec<Target> {
    let mut reachable_targets = Vec::new();
    for target in possible_targets {
        let adjacent_caverns =
            map.find_adjacent_by_type(target.position, MapElement::Terrain(Terrain::Cavern));
        let (closest_cavern, cavern_distance) = adjacent_caverns
            .iter()
            .map(|cavern| (cavern, map.find_distance(unit.position, *cavern)))
            .filter(|(_, distance)| distance.is_some())
            .min_by(|(_, distance_a), (_, distance_b)| distance_a.cmp(distance_b))
            .unwrap_or((&target.position, None));
        if let Some(distance) = cavern_distance {
            let target = Target {
                unit: *target,
                position: *closest_cavern,
                distance,
            };
            reachable_targets.push(target);
        }

        if map.manhattan_distance(unit.position, target.position) == 1 {
            let target = Target {
                unit: *target,
                position: target.position,
                distance: 0,
            };
            reachable_targets.push(target);
        }
    }
    reachable_targets
}

fn do_unit_action(map: &mut Map, unit: Unit, action: UnitAction) -> Position {
    match action {
        UnitAction::Move(target) => {
            return map.move_unit(unit, target);
        }
        UnitAction::Attack(target, buff) => {
            map.attack_unit(unit, target, buff);
            return unit.position;
        }
        UnitAction::None => {
            println!("Unit ({}, {}) has no action to do", unit.position.0, unit.position.1);
            return unit.position;
        }
    }
}
