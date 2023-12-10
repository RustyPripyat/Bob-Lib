use std::mem::discriminant;

use robotics_lib::interface::{robot_view, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;

pub fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

// pub fn get_tile_in_direction(
//     robot: &impl Runnable,
//     world: &World,
//     direction: &Direction,
// ) -> Option<Tile> {
//     let view = robot_view(robot, world);
//
//     let (robot_row, robot_col) = (
//         robot.get_coordinate().get_row(),
//         robot.get_coordinate().get_col(),
//     );
//     let (delta_row, delta_col) = match direction {
//         Direction::Up => (-1, 0),
//         Direction::Down => (1, 0),
//         Direction::Left => (0, -1),
//         Direction::Right => (0, 1),
//     };
//
//     let target_row = robot_row as isize + delta_row;
//     let target_col = robot_col as isize + delta_col;
//
//     // Check if the target coordinates are within bounds
//     if target_row >= 0 && target_col >= 0 {
//         let target_row = target_row as usize;
//         let target_col = target_col as usize;
//
//         view[target_row][target_col].clone()
//     } else {
//         None
//     }
// }

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

// return (energy, material)
pub fn get_edge_cost(tile: TileType) -> Option<(isize, isize)> {
    match tile {
        TileType::Grass | TileType::Sand | TileType::Hill | TileType::Snow => Some((1, 1)),
        TileType::ShallowWater => Some((2, 2)),
        TileType::DeepWater => Some((6, 3)),
        TileType::Lava => Some((9, 3)),
        TileType::Mountain => Some((16, -4)),
        _ => None,
    }
}

/// Calculates the cost relation based on energy, materials, and a divider.
///
/// This function computes the cost relation using the provided energy, materials, and a divider.
/// It divides the energy by the ratio of materials divided by the divider.
/// If either materials or energy is zero or less, the function returns 0.0.
///
/// # Arguments
///
/// * `energy` - An unsigned integer representing the amount of energy.
/// * `materials` - An unsigned integer representing the amount of materials.
/// * `divider` - A floating-point number used as a divider in the computation.
///
/// # Returns
///
/// Returns the calculated cost relation as a floating-point number.
///
pub fn costs_relation(energy: usize, materials: usize, divider: f64) -> f64 {
    match (materials, energy) {
        (0, _) | (_, 0) => 0.0,
        _ => {
            let en = energy as f64;
            let mat = materials as f64;
            en / (mat / divider)
        }
    }
}

/// Generates a String representation of a grid of tiles.
/// Each tile is formatted and printed within a Markdown-like table structure.
///
/// # Arguments
///
/// * `tiles` - A vector of vectors representing the grid of tiles.
///
pub fn pretty_print_tilemap(tiles: Vec<Vec<Tile>>) {
    for row in tiles {
        let mut row_str = String::new();
        for tile in row {
            let content_display = match tile.content {
                Content::None => format!("{:?}", tile.tile_type),
                _ => format!("{:?}({})", tile.tile_type, tile.content),
            };
            row_str += &format!("| {:<8}\t", content_display); // Adjust the width as needed
        }
        println!("{}|", row_str);
    }
}

pub fn match_content_type_variant(lhs: Option<Content>, rhs: Option<Content>) -> bool {
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => discriminant(&lhs) == discriminant(&rhs),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_content_type_both_some_same_content() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Rock(1));
        assert!(match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_some_same_content_different_value() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Rock(2));
        assert!(match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_some_different_content() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Tree(2));
        assert!(!match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_one_none() {
        let lhs = Some(Content::Rock(1));
        let rhs = None;
        assert!(!match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_none() {
        let lhs: Option<Content> = None;
        let rhs: Option<Content> = None;
        assert!(!match_content_type_variant(lhs, rhs));
    }
}
