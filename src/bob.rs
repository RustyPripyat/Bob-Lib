use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{go, put, robot_map, robot_view, Direction};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType::Street;
use robotics_lib::world::World;
use std::any::{type_name, Any, TypeId};
use std::iter::empty;
use std::ops::Deref;
use std::rc::Rc;

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

trait BobPinTrait<T> {
    fn calculate(&self) -> T;
}
trait BobPinTraitEmpty {
    fn calculate(&self) -> Option<Box<dyn Any>>;
}
struct BobPin<T> {
    pin: T,
}

impl<T: Clone> BobPinTrait<T> for BobPin<T> {
    fn calculate(&self) -> T {
        self.pin.clone()
    }
}

struct BobPinEmpty<T: BobPinTrait<T>> {
    pin: T,
}

impl<T: BobPinTrait<T>> BobPinEmpty<T> {
    fn new(pin: T) -> Self {
        Self { pin }
    }
}

impl<T: BobPinTrait<T>> BobPinTraitEmpty for BobPinEmpty<T> {
    fn calculate(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.pin.calculate()))
    }
}

struct BobMap {
    map: Vec<Vec<(Option<Tile>, Option<Rc<dyn BobPinTraitEmpty>>)>>,
}

const BOB_MAP_CONST: BobMap = BobMap::init();

impl BobMap {
    pub(crate) fn init() -> BobMap {
        todo!()
    }
    pub(crate) fn update(&mut self, view: &Vec<Vec<Option<Tile>>>) {
        todo!();
    }

    fn add_pin<T: BobPinTrait<T>>(&mut self, pin: T) {
        self.map[0][0].1 = Some(Rc::new(BobPinEmpty::new(pin)));
        todo!()
    }

    fn calculate<T: BobPinTrait<T>>(&self, coordinates: (usize, usize)) -> Option<T> {
        let pin = self.map[coordinates.0][coordinates.1].1.unwrap();
        let res: Box<T> = pin.calculate().unwrap().downcast().ok()?;
        Some(*res)
    }
}

// ---------------------------------------------

pub fn bob_view(robot: &impl Runnable, world: &World) -> Vec<Vec<Option<Tile>>> {
    let view = robot_view(robot, world);
    BOB_MAP_CONST.update(&view);
    let s = BOB_MAP_CONST.calculate((1, 2)).unwrap();
    view
}

pub fn bob_map() -> Vec<Vec<(Option<Tile>, Option<Rc<dyn Any>>)>> {
    BOB_MAP_CONST.map.clone()
}

pub fn bob_long_view() {
    todo!()
}

pub fn bob_pin() {
    todo!()
}

pub fn bob_remove_pin() {
    todo!()
}
