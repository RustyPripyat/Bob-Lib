use std::any::Any;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

use robotics_lib::interface::{Direction, one_direction_view, robot_map, robot_view};
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;

struct MyRobot(Robot);

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
            (BobPinTypes::Market, BobPinTypes::Market) => true,
            (BobPinTypes::City, BobPinTypes::City) => true,
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
                if Rc::<(dyn Any + 'static)>::as_ptr(val1)
                    .eq(&Rc::<(dyn Any + 'static)>::as_ptr(val2))
                {
                    return true;
                }
                false
            }
            (_, _) => false,
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
    pub fn init(world: &World) -> BobMap {
        let robot_map = robot_map(world).unwrap();
        let map: Vec<Vec<(Option<Tile>, Option<Rc<BobPinTypes>>)>> = robot_map
            .into_iter()
            .map(|row| row.into_iter().map(|tile| (tile, None)).collect())
            .collect();
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

pub fn bob_view<T: Clone>(
    robot: &impl Runnable,
    world: &World,
    map: &mut BobMap,
) -> Vec<Vec<Option<Tile>>> {
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

pub fn bob_long_view(
    robot: &mut impl Runnable,
    world: &World,
    direction: Direction,
    distance: usize,
    map: &mut BobMap,
) -> Result<Vec<Vec<Tile>>, LibError> {
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
