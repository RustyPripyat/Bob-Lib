use std::any::{Any, type_name, TypeId};
use std::io::Seek;
use std::iter::empty;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, put, Direction, robot_map, robot_view};
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

// ---------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum BobPinTypes{
    I32(i32),
    String(String),
    F64(f64),
    TileType(TileType),
    Contents(Content),
    City,
    Bank(usize),
    Market,
    Custom1(Rc<dyn Any>),
}

pub struct BobMap {
    map: Vec<Vec<(Option<Tile>, Option<Rc<BobPinTypes>>)>>
}

impl BobMap {
    pub fn init() -> BobMap {
        BobMap {
            map: vec![vec![(None, None)]]
        }
    }

    fn update(&mut self, view: &Vec<Vec<Option<Tile>>>) {
        todo!();
    }

    fn add_pin(&mut self, pin: Rc<BobPinTypes>) {
        self.map[0][0].1 = Some(pin.clone());
    }

    fn get_pin(&self) -> Rc<BobPinTypes> {
        self.map[0][0].1.as_ref().unwrap().clone()
    }
}

// ---------------------------------------------

pub fn bob_view<T: Clone>(robot: &impl Runnable, world: &World) -> Vec<Vec<Option<Tile>>> {
    let view = robot_view(robot, world);
    let mut map = BobMap::init();

    view
}

pub fn bob_map() -> Vec<Vec<(Option<Tile>, Option<Rc<dyn Any>>)>> {
    todo!()
}

pub fn bob_long_view() {
    todo!()
}

pub fn bob_pin(map: &mut BobMap, pin: BobPinTypes) {
    map.add_pin(Rc::new(pin));
}

pub fn bob_get_pin(map: &mut BobMap) -> BobPinTypes {
    map.get_pin().deref().clone()
}

pub fn bob_remove_pin() {
    todo!()
}

pub fn bob_type_check<T: 'static>(to_check: Rc<dyn Any>) -> Result<Rc<T>, ()> {
    if let Some(val) = to_check.downcast::<T>().ok(){
        return Ok(val);
    }
    Err(())
}