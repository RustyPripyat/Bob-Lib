use robotics_lib::interface::{robot_view, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::World;

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
