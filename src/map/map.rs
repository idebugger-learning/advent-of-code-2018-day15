use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnitType {
    Goblin,
    Elf,
}

pub type UnitDetails = (UnitType, u16);

#[derive(Debug, PartialEq)]
pub enum Terrain {
    Wall,
    Cavern,
}

#[derive(Debug, PartialEq)]
pub enum MapElement {
    Terrain(Terrain),
    Unit(UnitDetails),
}

#[derive(Debug)]
pub struct Map {
    pub(super) map: Vec<MapElement>,
    pub(super) width: usize,
}

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub details: UnitDetails,
    pub position: Position,
}

impl UnitType {
    pub fn get_enemy_type(&self) -> UnitType {
        match self {
            UnitType::Goblin => UnitType::Elf,
            UnitType::Elf => UnitType::Goblin,
        }
    }
}

impl Map {
    /**
     * Returns the positions of all units on the map in reading order.
     */
    pub fn get_units_positions(&self) -> Vec<Unit> {
        self.map
            .iter()
            .enumerate()
            .filter_map(|(i, element)| match element {
                MapElement::Unit(unit) => Some(Unit {
                    details: *unit,
                    position: (i % self.width, i / self.width),
                }),
                _ => None,
            })
            .collect()
    }

    pub fn get_unit(&self, position: Position) -> Option<Unit> {
        let index = position.1 * self.width + position.0;
        match self.map.get(index) {
            Some(MapElement::Unit(unit)) => Some(Unit {
                details: *unit,
                position,
            }),
            _ => None,
        }
    }

    pub fn find_path(&self, start: Position, end: Position) -> Option<Vec<Position>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parents = HashMap::new();
        queue.push_back((start, 0));
        while let Some((position, distance)) = queue.pop_front() {
            if position == end {
                let mut path = vec![position];
                let mut current = position;
                while let Some(parent) = parents.get(&current) {
                    path.push(*parent);
                    current = *parent;
                }
                path.reverse();
                return Some(path);
            }
            for neighbour in
            self.find_adjacent_by_type(position, MapElement::Terrain(Terrain::Cavern))
            {
                if visited.contains(&neighbour) {
                    continue;
                }
                visited.insert(neighbour);
                parents.insert(neighbour, position);
                queue.push_back((neighbour, distance + 1));
            }
        }
        None
    }

    pub fn find_distance(&self, start: Position, end: Position) -> Option<usize> {
        self.find_path(start, end).map(|path| path.len() - 1)
    }

    pub fn move_unit(&mut self, unit: Unit, position: Position) -> Position {
        let (x, y) = unit.position;
        let MapElement::Unit(_) = self.map[y * self.width + x] else {
            panic!("Unit not found at position ({}, {})", x, y);
        };

        let (new_x, new_y) = position;
        let MapElement::Terrain(Terrain::Cavern) = self.map[new_y * self.width + new_x] else {
            panic!("Unit cannot move to position ({}, {}) as it is not a cavern", new_x, new_y);
        };

        let path = self.find_path(unit.position, position).unwrap();
        let (new_x, new_y) = path[1];

        self.map[y * self.width + x] = MapElement::Terrain(Terrain::Cavern);
        self.map[new_y * self.width + new_x] = MapElement::Unit(unit.details);

        println!("Unit moved from ({}, {}) to ({}, {})", x, y, new_x, new_y);

        (new_x, new_y)
    }

    pub fn attack_unit(&mut self, unit: Unit, target: Position, buff: u16) {
        let (x, y) = unit.position;
        let MapElement::Unit(_) = self.map[y * self.width + x] else {
            panic!("Unit ({}, {}) cannot attack as it is not a unit", x, y);
        };

        let (target_x, target_y) = target;
        let MapElement::Unit(_) = self.map[target_y * self.width + target_x] else {
            panic!("Unit cannot attack unit at position ({}, {}) as it is not a unit", target_x, target_y);
        };

        let (target_type, target_hp) = match self.map[target_y * self.width + target_x] {
            MapElement::Unit((unit_type, hp)) => (unit_type, hp),
            _ => unreachable!("We have checked that the target is a unit"),
        };

        let distance = self.manhattan_distance(unit.position, target);
        if distance > 1 {
            panic!(
                "Unit ({}, {}) cannot attack unit at position ({}, {}) as it is not adjacent",
                x, y, target_x, target_y
            );
        }

        let damage = 3 + buff;
        if target_hp <= damage {
            self.map[target_y * self.width + target_x] = MapElement::Terrain(Terrain::Cavern);
        } else {
            self.map[target_y * self.width + target_x] =
                MapElement::Unit((target_type, target_hp - damage));
        }

        println!(
            "Unit at ({}, {}) attacked unit at ({}, {}) for {} damage",
            x, y, target_x, target_y, damage
        );
    }

    pub fn manhattan_distance(&self, start: Position, end: Position) -> usize {
        let (x1, y1) = start;
        let (x2, y2) = end;
        ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
    }

    pub fn find_adjacent_by_type(
        &self,
        position: Position,
        filter_type: MapElement,
    ) -> Vec<Position> {
        let (x, y) = position;
        let mut adjacent = vec![];
        if y > 0 {
            if self.map[(y - 1) * self.width + x] == filter_type {
                adjacent.push((x, y - 1));
            }
        }
        if x > 0 {
            if self.map[y * self.width + x - 1] == filter_type {
                adjacent.push((x - 1, y));
            }
        }
        if x < self.width - 1 {
            if self.map[y * self.width + x + 1] == filter_type {
                adjacent.push((x + 1, y));
            }
        }
        if y < self.map.len() / self.width - 1 {
            if self.map[(y + 1) * self.width + x] == filter_type {
                adjacent.push((x, y + 1));
            }
        }
        adjacent
    }

    pub fn total_hp(&self) -> usize {
        self.map
            .iter()
            .filter_map(|element| match element {
                MapElement::Unit((_, hp)) => Some(*hp as usize),
                _ => None,
            })
            .sum()
    }
}

impl Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitType::Goblin => write!(f, "G"),
            UnitType::Elf => write!(f, "E"),
        }
    }
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Wall => write!(f, "#"),
            Terrain::Cavern => write!(f, "."),
        }
    }
}

impl Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapElement::Terrain(terrain) => write!(f, "{}", terrain),
            MapElement::Unit((unit, _)) => write!(f, "{}", unit),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_rows = self
            .map
            .iter()
            .enumerate()
            .fold(
                Vec::new(),
                |mut acc: Vec<Vec<&MapElement>>, (i, element)| {
                    if i % self.width as usize == 0 {
                        acc.push(Vec::new());
                    }
                    acc.last_mut().unwrap().push(element);
                    acc
                },
            )
            .iter()
            .map(|row| {
                let mut extras = vec![];
                row.iter()
                    .map(|element| {
                        if let MapElement::Unit(_) = element {
                            extras.push(element);
                        }
                        format!("{}", element)
                    })
                    .collect::<Vec<String>>()
                    .join("")
                    .to_string()
                    + &"   "
                    + &extras
                    .iter()
                    .map(|element| match element {
                        MapElement::Unit((unit, hp)) => format!("{}({})", unit, hp),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .collect::<Vec<String>>();

        write!(f, "{}", formatted_rows.join("\n"))?;
        Ok(())
    }
}
