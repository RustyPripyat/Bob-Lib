use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, put, Direction, robot_map};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::TileType::Street;
use robotics_lib::world::World;


struct MyRobot(Robot);

// cant destroy list: Bin, Crate, Bank, Market, Building, Scarecrow

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // let rocks_number = self
        //     .get_backpack()
        //     .get_contents()
        //     .get(&Rock(1))
        //     .unwrap_or(&0);
        //
        // let direction = Direction::Up;

        // let point_a = (0, 0);
        // let point_b = (10, 10);

        // 1 rock removed per street tile
        // let tiles = robot_view(self, world);

        // match put(self, world, Rock(1), 1, direction) {
        //     Ok(quantity_put) => {
        //         println!("Successfully put {} items", quantity_put);
        //         // Continue with your program logic using the returned quantity
        //     }
        //     Err(err) => {
        //         eprintln!("Error: {:?}", err);
        //         // Handle the error case
        //     }
        // }
    }

    fn handle_event(&mut self, event: Event) {
        println!();
        println!("{:?}", event);
        println!();
    }

    fn get_energy(&self) -> &Energy {
        &self.0.energy
    }
    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.0.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.0.coordinate
    }
    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.0.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.0.backpack
    }
    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.0.backpack
    }
}

/// Calculates the sequence of directions to navigate from point A to point B in a rectangular manner.
///
/// # Arguments
///
/// * `a` - A tuple representing the starting point as `(row, col)`
/// * `b` - A tuple representing the ending point as `(row, col)`
///
/// # Returns
///
/// A vector containing the sequence of `Direction` enums to move from point A to point B.
///
/// # Examples
///
/// ```
/// use robotics_lib::interface::Direction;
/// use bob::navigate_rectangular;
///
/// let start = (1, 1);
/// let end = (4, 3);
/// let path = navigate_rectangular(start, end);
/// assert_eq!(path, vec![Direction::Right, Direction::Right, Direction::Right, Direction::Down, Direction::Down]);
/// ```
pub fn navigate_rectangular(a: (usize, usize), b: (usize, usize)) -> Vec<Direction> {
    let mut directions = Vec::new();
    // rows
    match a.0.cmp(&b.0) {
        std::cmp::Ordering::Less => directions.extend(vec![Direction::Up; b.0 - a.0]),
        std::cmp::Ordering::Greater => directions.extend(vec![Direction::Down; a.0 - b.0]),
        _ => (), // No horizontal movement needed
    }

    // columns
    match a.1.cmp(&b.1) {
        std::cmp::Ordering::Less => directions.extend(vec![Direction::Right; b.1 - a.1]),
        std::cmp::Ordering::Greater => directions.extend(vec![Direction::Left; a.1 - b.1]),
        _ => (), // No vertical movement needed
    }

    directions
}

/// Moves the robot and lays down streets or performs actions based on the given target and moves already executed.
///
/// # Arguments
///
/// * `robot` - A mutable reference to the robot that implements the `Runnable` trait.
/// * `world` - A mutable reference to the world.
/// * `target` - A tuple representing the target coordinates (row, col) to reach.
/// * `moves_already_done` - The number of moves already executed, used to resume the movement from a specific point.
///
/// # Returns
///
/// * `Result<(), LibError>` - Returns `Ok(())` if the movement and actions are successful, otherwise returns a `LibError`.
///
/// # Errors
///
/// The function can return errors of type `LibError` based on the actions performed during movement. Errors can include:
///
/// * `WrongContentUsed` - Occurs when incorrect content is used during the movement or action.
/// * `NotEnoughEnergy` - Occurs when the robot does not have enough energy to perform the action.
/// * `OperationNotAllowed` - Occurs when an operation is attempted but not allowed in the current context.
pub fn go_trace_street(
    robot: &mut impl Runnable,
    world: &mut World,
    target: (usize, usize),
    moves_already_done: u32,
) -> Result<(), LibError> {
    let start = (
        robot.get_coordinate().get_row(),
        robot.get_coordinate().get_col(),
    );
    let path = navigate_rectangular(start, target);
    let path_len = path.len();

    // now path is consumed
    let iter_path = path.into_iter().skip(moves_already_done as usize);

    let rocks_number = robot
        .get_backpack()
        .get_contents()
        .get(&Rock(1))
        .unwrap_or(&0);

    let rock_needed = path_len - moves_already_done as usize;

    if *rocks_number < rock_needed {
        return Err(LibError::NotEnoughContentInBackPack);
    }

    // todo!("Should check the Content of the tile in front of me and decide what to do. Maybe avoid it if it is not destroyable")


    for direction in iter_path {
        // get the tile in front of me
        match utils::get_tile_in_direction(robot, world, &direction) {
            Some(tile) => {
                // if the tile is a street, move
                match tile.tile_type {
                    Street => match go(robot, world, direction) {
                        Ok(_) => (), // Move successful, continue iterating
                        Err(err) => {
                            println!("Error encountered moving: {:?}", err);
                            return Err(err); // Or handle the error appropriately
                        }
                    },
                    _ => match put(robot, world, Rock(1), 1, direction) {
                        Ok(_) => (), // Move successful, continue iterating
                        Err(err) => {
                            println!("Error encountered laying rocks: {:?}", err);
                            return Err(err); // Or handle the error appropriately
                        }
                    },
                }
            }
            None => {
                // if the tile is not a street, put a street
                match put(robot, world, Rock(1), 1, direction) {
                    Ok(_) => (), // Move successful, continue iterating
                    Err(err) => {
                        // Handle the error returned by put function
                        println!("Error encountered laying rocks: {:?}", err);
                        // You can choose to break the loop, return an error, or take other actions
                        return Err(err);
                    }
                }
            }
        }
    }
    Ok(())
}
