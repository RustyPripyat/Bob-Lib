// put out fires
// get x items
// sell x things
// reach x location

// ---------------------------------------------------------------------

use std::fmt;
use std::fmt::Display;

use robotics_lib::interface::{destroy, put, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;

use utils::get_tile_in_direction;

/// Enum representing various types of goals in a robotics context.
#[derive(Debug)]
pub enum GoalType {
    PutOutFire,
    GetItems,
    SellItems,
    // ReachLocation,
    ThrowGarbage,
}

#[derive(Debug)]
struct Goal {
    name: String,
    description: String,
    goal_type: GoalType,
    completed: bool,
    goal_quantity: usize,
    items_left: usize,
}

impl Goal {
    pub fn new(
        name: String,
        description: String,
        goal_type: GoalType,
        goal_quantity: usize,
    ) -> Goal {
        Goal {
            name,
            description,
            goal_type,
            completed: false,
            goal_quantity,
            items_left: goal_quantity,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_goal_type(&self) -> &GoalType {
        &self.goal_type
    }

    pub fn get_completed(&self) -> &bool {
        &self.completed
    }

    pub fn get_goal_quantity(&self) -> &usize {
        &self.goal_quantity
    }

    pub fn get_items_left(&self) -> &usize {
        &self.items_left
    }
}

impl Display for Goal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let completed_status = if self.completed {
            "Completed"
        } else {
            "Not Completed"
        };

        write!(
            f,
            "Goal Details:\nName: {}\nDescription: {}\nType: {:?}\nStatus: {}\nGoal Quantity: {}\nItems Left: {}",
            self.name, self.description, self.goal_type, completed_status, self.goal_quantity, self.items_left
        )
    }
}

struct GoalTracker {
    goals: Vec<Goal>,
    completed_number: usize,
}

impl GoalTracker {
    pub fn new() -> GoalTracker {
        GoalTracker {
            goals: Vec::new(),
            completed_number: 0,
        }
    }
    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
    }

    pub fn get_goals(&self) -> &Vec<Goal> {
        &self.goals
    }

    pub fn get_completed_number(&self) -> usize {
        self.completed_number
    }

    pub fn udpate(&mut self, result: Result<(), LibError>, rhs_goal_type: GoalType) {
        match result {
            Ok(_) => {
                for goal in self.goals.iter_mut() {
                    if goal.goal_type == rhs_goal_type {
                        goal.items_left -= 1;
                        if goal.items_left == 0 {
                            goal.completed = true;
                            self.completed_number += 1;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Err(LibError::new("Error: Goal not completed"))
            }
        }
    }
}

impl Display for GoalTracker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the formatting logic here
        write!(
            f,
            "GoalTracker:\nGoals: {:?}\nCompleted: {}",
            self.goals, self.completed_number
        )
    }
}

pub fn put_out_fire(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    mut goal_tracker: GoalTracker,
) -> Result<(), LibError> {
    // check if robot is in front of fire
    match get_tile_in_direction(robot, world, &direction)
        .unwrap()
        .get_content()
    {
        Some(Content::Fire) => {}
        _ => {
            eprintln!("Error: {:?}", LibError::new("Error: not fire"));
            return Err(LibError::new("Error: not fire"));
        }
    }

    match put(robot, world, content_in, quantity, direction) {
        Ok(removed_quantity) => {
            // println!("Successfully put {} items", quantity_put);
            goal_tracker.udpate(Ok(()), GoalType::PutOutFire);
            // Continue with your program logic using the returned quantity
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            Err(LibError::new("Error: fire not put out"))
            // Handle the error case
        }
    }
    Ok(())
}

pub fn sell_items(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    mut goal_tracker: GoalTracker,
) -> Result<(), LibError> {
    // check if the robos is in front of market
    match get_tile_in_direction(robot, world, &direction)
        .unwrap()
        .get_content()
    {
        Some(Content::Market) => {}
        _ => {
            eprintln!("Error: {:?}", LibError::new("Error: not market"));
            return Err(LibError::new("Error: not market"));
        }
    }

    match put(robot, world, content_in, quantity, direction) {
        Ok(removed_quantity) => {
            // println!("Successfully put {} items", quantity_put);
            goal_tracker.udpate(Ok(()), GoalType::SellItems);
            // Continue with your program logic using the returned quantity
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            Err(LibError::new("Error: items not sold"))
            // Handle the error case
        }
    }
    Ok(())
}

pub fn throw_garbage(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    mut goal_tracker: GoalTracker,
) -> Result<(), LibError> {
    // check if the robot is in front of bin and content_in is garbage
    match get_tile_in_direction(robot, world, &direction)
        .unwrap()
        .get_content()
    {
        Some(Content::Bin) => {}
        _ => {
            eprintln!("Error: {:?}", LibError::new("Error: not garbage"));
            return Err(LibError::new("Error: not garbage"));
        }
    }

    match put(robot, world, content_in, quantity, direction) {
        Ok(removed_quantity) => {
            // println!("Successfully put {} items", quantity_put);
            goal_tracker.udpate(Ok(()), GoalType::ThrowGarbage);
            // Continue with your program logic using the returned quantity
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            Err(LibError::new("Error: garbage not thrown"))
            // Handle the error case
        }
    }
    Ok(())
}

pub fn get_items(
    robot: &mut impl Runnable,
    world: &mut World,
    direction: Direction,
    mut goal_tracker: GoalTracker,
) -> Result<(), LibError> {
    // check if the robot is in front of item
    match get_tile_in_direction(robot, world, &direction)
        .unwrap()
        .get_content()
    {
        Some(Content::None) => {
            eprintln!("Error: {:?}", LibError::new("Error: not item"));
            return Err(LibError::new("Error: not item"));
        }
        _ => {}
    }

    // destroy(robot,world, direction
    match destroy(robot, world, direction) {
        Ok(removed_quantity) => {
            // println!("Successfully put {} items", quantity_put);
            goal_tracker.udpate(Ok(()), GoalType::GetItems);
            // Continue with your program logic using the returned quantity
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
            Err(LibError::new("Error: items not gotten"))
            // Handle the error case
        }
    }
    Ok(())
}
