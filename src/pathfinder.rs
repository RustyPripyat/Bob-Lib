use std::cmp::min;
use std::fmt::Debug;
use pathfinding::num_traits::AsPrimitive;
use pathfinding::prelude::{
    dijkstra
};
use robotics_lib::world::tile::Tile;

// x, y, energy, rocks
struct Node(
    usize,
    usize,
    isize,
    isize,
);

enum BobMode {
    EnergySave,
    MaterialSave,
    AllOut,
}

//tutto da dinamicare
impl Node {
    fn successors(&self, map: &Vec<Vec<Option<Tile>>>, mode: BobMode, relation: f64) -> Vec<(Node, usize)> {
        let mut res: Vec<(Node, usize)> = vec![];
        let &Node(x, y, energy, rocks) = self;
        if let Some(up) = map[x][y + 1].as_ref() {
            if let Some(cost) = utils::get_edge_cost(up.tile_type) {
                let tot_rocks = min(
                    rocks - cost.1,
                    20,
                );
                match mode {
                    BobMode::EnergySave => {
                        let cost_energy = (cost.0 as f64 * relation).round().as_();
                        res.push((Node(x, y + 1, energy - cost.0, tot_rocks), cost_energy));
                    }
                    BobMode::MaterialSave => {
                        let cost_material = (cost.1 as f64 / relation).round().as_();
                        res.push((Node(x, y + 1, energy - cost.0, tot_rocks), cost_material));
                    }
                    BobMode::AllOut => {
                        let cost_energy = (cost.0 as f64 * relation).round() as usize;
                        let cost_material = (cost.1 as f64 / relation).round() as usize;
                        res.push((Node(x, y + 1, energy - cost.0, tot_rocks), cost_energy + cost_material));
                    }
                }
            }
        };
        let &Node(x, y, energy, rocks) = self;
        if let Some(down) = map[x][y - 1].as_ref() {
            if let Some(cost) = utils::get_edge_cost(down.tile_type) {
                let tot_rocks = min(
                    rocks - cost.1,
                    20,
                );
                match mode {
                    BobMode::EnergySave => {
                        let cost_energy = (cost.0 as f64 * relation).round().as_();
                        res.push((Node(x, y - 1, energy - cost.0, tot_rocks), cost_energy));
                    }
                    BobMode::MaterialSave => {
                        let cost_material = (cost.1 as f64 / relation).round().as_();
                        res.push((Node(x, y - 1, energy - cost.0, tot_rocks), cost_material));
                    }
                    BobMode::AllOut => {
                        let cost_energy = (cost.0 as f64 * relation).round() as usize;
                        let cost_material = (cost.1 as f64 / relation).round() as usize;
                        res.push((Node(x, y - 1, energy - cost.0, tot_rocks), cost_energy + cost_material));
                    }
                }
            }
        };
        let &Node(x, y, energy, rocks) = self;
        if let Some(left) = map[x - 1][y].as_ref() {
            if let Some(cost) = utils::get_edge_cost(left.tile_type) {
                let tot_rocks = min(
                    rocks - cost.1,
                    20,
                );
                match mode {
                    BobMode::EnergySave => {
                        let cost_energy = (cost.0 as f64 * relation).round().as_();
                        res.push((Node(x - 1, y, energy - cost.0, tot_rocks), cost_energy));
                    }
                    BobMode::MaterialSave => {
                        let cost_material = (cost.1 as f64 / relation).round().as_();
                        res.push((Node(x - 1, y, energy - cost.0, tot_rocks), cost_material));
                    }
                    BobMode::AllOut => {
                        let cost_energy = (cost.0 as f64 * relation).round() as usize;
                        let cost_material = (cost.1 as f64 / relation).round() as usize;
                        res.push((Node(x - 1, y, energy - cost.0, tot_rocks), cost_energy + cost_material));
                    }
                }
            }
        };
        let &Node(x, y, energy, rocks) = self;
        if let Some(right) = map[x + 1][y].as_ref() {
            if let Some(cost) = utils::get_edge_cost(right.tile_type) {
                let tot_rocks = min(
                    rocks - cost.1,
                    20,
                );
                match mode {
                    BobMode::EnergySave => {
                        let cost_energy = (cost.0 as f64 * relation).round().as_();
                        res.push((Node(x + 1, y, energy - cost.0, tot_rocks), cost_energy));
                    }
                    BobMode::MaterialSave => {
                        let cost_material = (cost.1 as f64 / relation).round().as_();
                        res.push((Node(x + 1, y, energy - cost.0, tot_rocks), cost_material));
                    }
                    BobMode::AllOut => {
                        let cost_energy = (cost.0 as f64 * relation).round() as usize;
                        let cost_material = (cost.1 as f64 / relation).round() as usize;
                        res.push((Node(x + 1, y, energy - cost.0, tot_rocks), cost_energy + cost_material));
                    }
                }
            }
        };
        res
    }
}

// let d = dijkstra(&Node(0, 0, 1000, 20), |n| n.successors(&robot_map(&world).unwrap()), |n| n == (10, 10));