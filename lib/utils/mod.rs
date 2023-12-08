use robotics_lib::energy::Energy;
use robotics_lib::interface::{robot_view, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::{Tile, TileType};
use robotics_lib::world::World;
use std::ops::Div;

pub fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

pub fn get_tile_in_direction(
    robot: &impl Runnable,
    world: &World,
    direction: &Direction,
) -> Option<Tile> {
    let view = robot_view(robot, world);

    let (robot_row, robot_col) = (
        robot.get_coordinate().get_row(),
        robot.get_coordinate().get_col(),
    );
    let (delta_row, delta_col) = match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    let target_row = robot_row as isize + delta_row;
    let target_col = robot_col as isize + delta_col;

    // Check if the target coordinates are within bounds
    if target_row >= 0 && target_col >= 0 {
        let target_row = target_row as usize;
        let target_col = target_col as usize;

        view[target_row][target_col].clone()
    } else {
        None
    }
}

// return (energy, material)
pub fn get_edge_cost(tile: TileType) -> Option<(isize, isize)> {
    match tile {
        (TileType::Grass | TileType::Sand | TileType::Hill | TileType::Snow) => Some((1, 1)),
        (TileType::ShallowWater) => Some((2, 2)),
        (TileType::DeepWater) => Some((6, 3)),
        (TileType::Lava) => Some((9, 3)),
        (TileType::Mountain) => Some((16, -4)),
        _ => None,
    }
}

pub fn costs_relation(energy: usize, materials: usize, divider: f64) -> f64 {
    let en = energy as f64;
    let mut mat = materials as f64;

    if materials <= 0 || energy <= 0 {
        0.0
    } else {
        en.div(mat.div(divider))
    }
}
