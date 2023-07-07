use nom::{
    branch::alt,
    bytes::complete::tag,
    multi::{many1, separated_list1},
    IResult,
};

use crate::{INITIAL_ELF_HP, INITIAL_GOBLIN_HP};

use super::map::{Map, MapElement, Terrain, UnitType};

pub fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, map) = separated_list1(tag("\n"), parse_line)(input)?;

    let width = map[0].len();
    let map = map.into_iter().flatten().collect();
    Ok((input, Map { map, width }))
}

fn parse_line(input: &str) -> IResult<&str, Vec<MapElement>> {
    let (input, map_line) = many1(parse_map_element)(input)?;

    Ok((input, map_line))
}

fn parse_map_element(input: &str) -> IResult<&str, MapElement> {
    alt((parse_wall, parse_cavern, parse_goblin, parse_elf))(input)
}

fn parse_wall(input: &str) -> IResult<&str, MapElement> {
    let (input, _) = tag("#")(input)?;

    Ok((input, MapElement::Terrain(Terrain::Wall)))
}

fn parse_cavern(input: &str) -> IResult<&str, MapElement> {
    let (input, _) = tag(".")(input)?;

    Ok((input, MapElement::Terrain(Terrain::Cavern)))
}

fn parse_goblin(input: &str) -> IResult<&str, MapElement> {
    let (input, _) = tag("G")(input)?;

    Ok((
        input,
        MapElement::Unit((UnitType::Goblin, INITIAL_GOBLIN_HP)),
    ))
}

fn parse_elf(input: &str) -> IResult<&str, MapElement> {
    let (input, _) = tag("E")(input)?;

    Ok((input, MapElement::Unit((UnitType::Elf, INITIAL_ELF_HP))))
}
