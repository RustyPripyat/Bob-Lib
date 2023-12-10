// use robotics_lib::runner::Robot;

use std::collections::HashMap;

use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::Direction::*;
use robotics_lib::interface::{debug, Tools};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable, Runner};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::Rock;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;

use utils::pretty_print_tilemap;

use crate::tracker::{destroy_and_collect_item, Goal, GoalType};

mod bob;
mod tracker;
// new code

fn main() {
    struct MyRobot(Robot);
    struct WorldGenerator {
        size: usize,
    }
    impl WorldGenerator {
        fn init(size: usize) -> Self {
            WorldGenerator { size }
        }
    }
    impl Generator for WorldGenerator {
        fn gen(
            &mut self,
        ) -> (
            Vec<Vec<Tile>>,
            (usize, usize),
            EnvironmentalConditions,
            f32,
            Option<HashMap<Content, f32>>,
        ) {
            let mut map: Vec<Vec<Tile>> = Vec::new();

            for _ in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for _ in 0..self.size {
                    row.push(Tile {
                        tile_type: TileType::Grass,
                        content: Content::None,
                        elevation: 0,
                    });
                }
                map.push(row);
            }

            map[0][1].content = Rock(1);
            map[0][2].content = Rock(1);
            map[0][3].content = Rock(1);

            let environmental_conditions =
                EnvironmentalConditions::new(&[Sunny, Rainy], 15, 12).unwrap();
            let max_score = rand::random::<f32>();

            (map, (0, 0), environmental_conditions, max_score, None)
        }
    }

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut World) {
            // println!("{:?}", robot_view(self, world))
            println!("{:?}", pretty_print_tilemap(debug(self, world).0));
            // println!("{:?}", one_direction_view(self, world, Right, 2));

            let mut tracker = tracker::GoalTracker::new();
            let get_3_rocks_goal = Goal::new(
                "rocks".to_string(),
                "rocks".to_string(),
                GoalType::GetItems,
                Some(Rock(3)),
                3,
            );

            tracker.add_goal(get_3_rocks_goal);

            println!("{:?}", tracker.get_goals());
            println!("BackPack: {:?}", self.get_backpack().get_contents());

            match destroy_and_collect_item(self, world, Right, &mut tracker, Some(Rock(1))) {
                Ok(size) => println!("Success, got {} rocks", size),
                Err(e) => println!("Error: {:?}", e),
            }

            println!("BackPack: {:?}", self.get_backpack().get_contents());
            println!("{:?}", tracker.get_goals());

            println!("{:?}", pretty_print_tilemap(debug(self, world).0));
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

    let r = MyRobot(Robot::new());
    struct Tool;
    impl Tools for Tool {}
    let mut generator = WorldGenerator::init(5);
    let run = Runner::new(Box::new(r), &mut generator);

    //Known bug: 'check_world' inside 'Runner::new()' fails every time
    match run {
        Ok(mut r) => {
            let _ = r.game_tick();
        }
        Err(e) => println!("{:?}", e),
    }
}
