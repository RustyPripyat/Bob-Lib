use std::mem::discriminant;
use robotics_lib::interface::{Direction, robot_view};
use robotics_lib::runner::Runnable;

use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::World;

pub fn match_content_type_variant(lhs: Option<Content>, rhs: Option<Content>) -> bool {
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => discriminant(&lhs) == discriminant(&rhs),
        _ => false,
    }
}

pub fn get_tile_in_direction(
    robot: &mut impl Runnable,
    world: &mut World,
    direction: &Direction,
) -> Option<Tile> {
    let view = robot_view(robot, world);
    let center = (view.len() / 2, view[0].len() / 2);
    let (mut row, mut col) = center;

    match direction {
        Direction::Up => row -= 1,
        Direction::Down => row += 1,
        Direction::Left => col -= 1,
        Direction::Right => col += 1,
    }

    if let Some(tile) = view.get(row) {
        if let Some(tile) = tile.get(col) {
            return tile.clone();
        }
    }

    None
}