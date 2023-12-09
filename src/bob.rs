use std::any::{Any, type_name, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::iter::empty;
use std::ops::Deref;
use std::rc::Rc;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, put, Direction, robot_map, robot_view, one_direction_view};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::{Content, Tile, TileType};
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

#[derive(Debug, Clone)]
pub enum BobPinTypes {
    I32(i32),
    String(String),
    TileType(TileType),
    Contents(Content),
    City,
    Bank(usize),
    Market,
    Custom(Rc<dyn Any>),
}

impl PartialEq<Self> for BobPinTypes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BobPinTypes::Market, BobPinTypes::Market) => {
                true
            }
            (BobPinTypes::City, BobPinTypes::City) => {
                true
            }
            (BobPinTypes::I32(val1), BobPinTypes::I32(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::String(val1), BobPinTypes::String(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::TileType(val1), BobPinTypes::TileType(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Contents(val1), BobPinTypes::Contents(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Bank(val1), BobPinTypes::Bank(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Custom(val1), BobPinTypes::Custom(val2)) => {
                if Rc::<(dyn Any + 'static)>::as_ptr(val1).eq(&Rc::<(dyn Any + 'static)>::as_ptr(val2)) {
                    return true;
                }
                false
            }
            (_, _) => false
        }
    }
}

impl Eq for BobPinTypes {}

impl Hash for BobPinTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match &self {
            BobPinTypes::I32(value) => value.hash(state),
            BobPinTypes::String(value) => value.hash(state),
            BobPinTypes::TileType(value) => value.hash(state),
            BobPinTypes::Contents(value) => value.hash(state),
            BobPinTypes::City => (),
            BobPinTypes::Bank(value) => value.hash(state),
            BobPinTypes::Market => (),
            BobPinTypes::Custom(rc_any) => {
                let ptr = Rc::<(dyn Any + 'static)>::as_ptr(&rc_any);
                ptr.hash(state)
            }
        }
    }
}

pub struct BobMap {
    map: Vec<Vec<(Option<Tile>, Option<Rc<BobPinTypes>>)>>,
    saved_pins: HashMap<BobPinTypes, Vec<(usize, usize)>>,
}

impl BobMap {
    pub fn init(/*world: &World*/) -> BobMap {
        // let robot_map = robot_map(world).unwrap();
        let robot_map = vec![
            vec![Some(Tile {
                tile_type: TileType::Grass,
                content: Content::Fire,
                elevation: 0,
            }), Some(Tile {
                tile_type: TileType::Grass,
                content: Content::None,
                elevation: 0,
            })],
            vec![Some(Tile {
                tile_type: TileType::Hill,
                content: Content::Coin(2),
                elevation: 0,
            }), Some(Tile {
                tile_type: TileType::Hill,
                content: Content::Coin(5),
                elevation: 0,
            })],
            vec![Some(Tile {
                tile_type: TileType::Street,
                content: Content::Bin(0..3),
                elevation: 0,
            }), Some(Tile {
                tile_type: TileType::Lava,
                content: Content::None,
                elevation: 0,
            })],
            vec![Some(Tile {
                tile_type: TileType::Teleport(false),
                content: Content::None,
                elevation: 0,
            }), Some(Tile {
                tile_type: TileType::Grass,
                content: Content::Garbage(2),
                elevation: 0,
            })],
        ];
        let map: Vec<Vec<(Option<Tile>, Option<Rc<BobPinTypes>>)>> = robot_map.into_iter().map(
            |row| row.into_iter().map(
                |tile| (tile, None)
            ).collect()
        ).collect();
        BobMap {
            map,
            saved_pins: HashMap::new(),
        }
    }

    fn update(&mut self, coordinates: Vec<(usize, usize, Tile)>) {
        for (x, y, tile) in coordinates {
            self.map[x][y].0 = Some(tile);
        }
    }

    pub fn add_pin(&mut self, pin: Rc<BobPinTypes>, (x, y): (usize, usize)) {
        self.map[x][y].1 = Some(pin.clone());
        if self.saved_pins.contains_key(&pin) {
            let vec = self.saved_pins.get_mut(&pin).unwrap();
            vec.push((x, y));
        } else {
            self.saved_pins.insert(pin.deref().clone(), vec![(x, y)]);
        }
    }

    pub fn get_pin(&self, (x, y): (usize, usize)) -> Option<Rc<BobPinTypes>> {
        self.map[x][y].1.clone().clone()
    }

    pub fn delete_pin(&mut self, (x, y): (usize, usize)) -> Result<(), ()> {
        if self.map[x][y].1.is_some() {
            self.map[x][y].1 = None;
            return Ok(());
        }
        Err(())
    }

    pub fn get_map(&self) -> &Vec<Vec<(Option<Tile>, Option<Rc<BobPinTypes>>)>> {
        self.map.as_ref()
    }

    pub fn search_pin(&self, pin: Rc<BobPinTypes>) -> Option<Vec<(usize, usize)>> {
        if self.saved_pins.contains_key(&pin) {
            let vec = self.saved_pins.get(&pin).unwrap();
            return Some(vec.clone());
        } else {
            None
        }
    }
}

pub fn bob_view<T: Clone>(robot: &impl Runnable, world: &World, map: &mut BobMap) -> Vec<Vec<Option<Tile>>> {
    let view = robot_view(robot, world);
    let pos = robot.get_coordinate();
    let mut update_vector: Vec<(usize, usize, Tile)> = vec![];

    for (i, v) in view.iter().enumerate() {
        for (j, tile) in v.iter().enumerate() {
            if tile.is_some() {
                let x = pos.get_row() - 1 + i;
                let y = pos.get_col() - 1 + j;
                update_vector.push((x, y, tile.clone().unwrap()));
            }
        }
    }

    map.update(update_vector);
    view
}

pub fn bob_long_view(robot: &mut impl Runnable, world: &World, direction: Direction, distance: usize, map: &mut BobMap) -> Result<Vec<Vec<Tile>>, LibError> {
    let long_view = one_direction_view(robot, world, direction.clone(), distance)?;
    let mut update_vector: Vec<(usize, usize, Tile)> = vec![];
    let pos = robot.get_coordinate();

    match direction {
        Direction::Up => {
            for (i, v) in long_view.iter().enumerate() {
                for (j, tile) in v.iter().enumerate() {
                    let x = pos.get_row() - 1 - i;
                    let y;
                    if pos.get_col() == 0 {
                        y = pos.get_col() + j;
                    } else {
                        y = pos.get_col() - 1 + j;
                    }

                    update_vector.push((x, y, tile.clone()));
                }
            }
        }
        Direction::Down => {
            for (i, v) in long_view.iter().enumerate() {
                for (j, tile) in v.iter().enumerate() {
                    let x = pos.get_row() - 1 + i;
                    let y;
                    if pos.get_col() == 0 {
                        y = pos.get_col() + j;
                    } else {
                        y = pos.get_col() - 1 + j;
                    }

                    update_vector.push((x, y, tile.clone()));
                }
            }
        }
        Direction::Left => {
            for (i, v) in long_view.iter().enumerate() {
                for (j, tile) in v.iter().enumerate() {
                    let y = pos.get_col() - 1 - j;
                    let x;
                    if pos.get_row() == 0 {
                        x = pos.get_row() + i;
                    } else {
                        x = pos.get_row() - 1 + i;
                    }
                    update_vector.push((x, y, tile.clone()));
                }
            }
        }
        Direction::Right => {
            for (i, v) in long_view.iter().enumerate() {
                for (j, tile) in v.iter().enumerate() {
                    let y = pos.get_col() - 1 + j;
                    let x;
                    if pos.get_row() == 0 {
                        x = pos.get_row() + i;
                    } else {
                        x = pos.get_row() - 1 + i;
                    }
                    update_vector.push((x, y, tile.clone()));
                }
            }
        }
    }

    map.update(update_vector);
    Ok(long_view)
}

pub fn bob_type_check<T: 'static>(to_check: Rc<dyn Any>) -> Result<Rc<T>, ()> {
    if let Some(val) = to_check.downcast::<T>().ok() {
        return Ok(val);
    }
    Err(())
}