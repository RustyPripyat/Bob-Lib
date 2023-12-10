use std::fmt;
use std::fmt::Display;

use robotics_lib::interface::{destroy, put, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;

use utils::{get_tile_in_direction, match_content_type_variant};

/// Enum representing various types of goals in a robotics context.
#[derive(Debug, PartialEq)]
pub enum GoalType {
    PutOutFire,
    GetItems,
    SellItems,
    ThrowGarbage,
}

#[derive(Debug)]
pub struct Goal {
    name: String,
    description: String,
    goal_type: GoalType,
    item_type: Option<Content>,
    completed: bool,
    goal_quantity: u32,
    items_left: u32,
}

impl Goal {
    pub fn new(
        name: String,
        description: String,
        goal_type: GoalType,
        item_type: Option<Content>,
        goal_quantity: u32,
    ) -> Goal {
        Goal {
            name,
            description,
            goal_type,
            item_type,
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

    pub fn get_goal_quantity(&self) -> &u32 {
        &self.goal_quantity
    }

    pub fn get_items_left(&self) -> &u32 {
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

pub struct GoalTracker {
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

    /// Remove a goal from the tracker based on its name.
    ///
    /// # Arguments
    /// * `goal_name` - The name of the goal to be removed.
    ///
    /// # Returns
    /// Option<Goal> - The removed goal if found, None otherwise.
    pub fn remove_goal(&mut self, goal_name: &str) -> Option<Goal> {
        if let Some(index) = self.goals.iter().position(|goal| goal.name == goal_name) {
            let removed_goal = self.goals.remove(index);
            if removed_goal.completed {
                self.completed_number -= 1;
            }
            Some(removed_goal)
        } else {
            None
        }
    }

    pub fn clean_completed_goals(&mut self) {
        self.goals.retain(|goal| !goal.completed);
    }

    pub fn get_goals(&self) -> &Vec<Goal> {
        &self.goals
    }

    pub fn get_completed_number(&self) -> usize {
        self.completed_number
    }

    /// Update the goal tracker based on the action result and the corresponding goal type.
    /// Only the first goal with the same goal type and item type will be updated.
    ///
    /// # Arguments
    /// * `result` - Result of the action performed.
    /// * `rhs_goal_type` - The goal type to be updated.
    /// * `rhs_item_type` - The item type of the goal.
    fn update(
        &mut self,
        result: Result<(), LibError>,
        rhs_goal_type: GoalType,
        rhs_item_type: Option<Content>,
        removed_quantity: usize,
    ) {
        if result.is_ok() {
            if let Some(goal) = self.goals.iter_mut().find(|goal| {
                goal.goal_type == rhs_goal_type
                    && match_content_type_variant(goal.item_type.clone(), rhs_item_type.clone())
            }) {
                println!("Found goal: {:?}", goal);
                goal.items_left -= removed_quantity as u32;
                goal.items_left = goal.items_left.max(0);
                // if negative value, set to 0
                if goal.items_left == 0 {
                    goal.completed = true;
                    self.completed_number += 1;
                }
            } else {
                eprintln!("Error: Goal not found");
            }
        } else if let Err(e) = result {
            eprintln!("Error: {:?}", e);
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

/// Puts out a fire in a specified direction by using the robot to perform the action.
/// It automatically checks if the robot is in front of a Fire and if the content is
/// valid. If not, it returns an error. It does update all your goals if the action is successful.
///
/// # Arguments
/// * `robot` - The robot that will perform the action.
/// * `world` - The world in which the action takes place.
/// * `content_in` - The type of content to put out the fire.
/// * `quantity` - The quantity of the content to use for putting out the fire.
/// * `direction` - The direction in which to perform the action.
/// * `goal_tracker` - The goal tracker to update upon successfully putting out the fire.
///
/// # Returns
/// Result<(usize), LibError> - Ok((removed_quantity)) if the action is successful, Err(LibError) otherwise.
///
pub fn put_out_fire(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    goal_tracker: &mut GoalTracker,
) -> Result<usize, LibError> {
    // check if robot is in front of fire
    match get_tile_in_direction(robot, world, &direction)
        .unwrap()
        .content
    {
        Content::Fire => {}
        _ => {
            let err = LibError::OperationNotAllowed;
            eprintln!("Error: {:?}", err);
            return Err(err);
        }
    }

    handle_put(
        robot,
        world,
        content_in,
        quantity,
        direction,
        goal_tracker,
        GoalType::PutOutFire,
    )
    // Ok(())
}

/// Sells items in a specified direction by using the robot to perform the action.
/// It automatically checks if the robot is in front of a market and if the content to sell is
/// valid. If not, it returns an error. It does update all your goals if the action is successful.
///
/// # Arguments
/// * `robot` - The robot that will perform the action.
/// * `world` - The world in which the action takes place.
/// * `content_in` - The type of content to sell.
/// * `quantity` - The quantity of the content to sell.
/// * `direction` - The direction in which to perform the action.
/// * `goal_tracker` - The goal tracker to update upon successfully selling items.
///
/// # Returns
/// Result<(usize), LibError> - Ok((removed_quantity)) if the action is successful, Err(LibError) otherwise.
///
pub fn sell_items_in_market(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    goal_tracker: &mut GoalTracker,
) -> Result<usize, LibError> {
    // check if the robot is in front of market
    if let Some(tile) = get_tile_in_direction(robot, world, &direction) {
        match tile.content {
            Content::Market(_) => {}
            _ => {
                let err = LibError::OperationNotAllowed;
                eprintln!("Error: {:?}", err);
                return Err(err);
            }
        }
    }

    handle_put(
        robot,
        world,
        content_in,
        quantity,
        direction,
        goal_tracker,
        GoalType::SellItems,
    )
}

/// Throws garbage in a specified direction by using the robot to perform the action.
/// It automatically checks if the robot is in front of a Bin and if the content to throw is
/// valid. If not, it returns an error. It does update all your goals if the action is successful.
///
/// # Arguments
/// * `robot` - The robot that will perform the action.
/// * `world` - The world in which the action takes place.
/// * `content_in` - The type of content (garbage) to throw.
/// * `quantity` - The quantity of the content to throw.
/// * `direction` - The direction in which to perform the action.
/// * `goal_tracker` - The goal tracker to update upon successfully throwing garbage.
///
/// # Returns
/// Result<(usize), LibError> - Ok((removed_quantity)) if the action is successful, Err(LibError) otherwise.
///
pub fn throw_garbage(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    goal_tracker: &mut GoalTracker,
) -> Result<usize, LibError> {
    // check if the robot is in front of bin and content_in is garbage
    match get_tile_in_direction(robot, world, &direction) {
        Some(tile) => match tile.content {
            Content::Bin(_) => {}
            _ => {
                let err = LibError::OperationNotAllowed;
                eprintln!("Error: {:?}", err);
                return Err(err);
            }
        },
        None => {
            let err = LibError::OutOfBounds;
            eprintln!("Error: {:?}", err);
            return Err(err);
        }
    }

    handle_put(
        robot,
        world,
        content_in,
        quantity,
        direction,
        goal_tracker,
        GoalType::ThrowGarbage,
    )
}

/// Gets items in a specified direction by using the robot to perform the action.
/// It automatically checks if the robot is in front of a Content. If not, it returns an error.
/// It does update all your goals if the action is successful.
///
/// # Arguments
/// * `robot` - The robot that will perform the action.
/// * `world` - The world in which the action takes place.
/// * `direction` - The direction in which to perform the action.
/// * `goal_tracker` - The goal tracker to update upon successfully getting items.
/// * `item_type` - The type of item to get.
///
/// # Returns
/// Result<(usize), LibError> - Ok((removed_quantity)) if the action is successful, Err(LibError) otherwise.
///
pub fn destroy_and_collect_item(
    robot: &mut impl Runnable,
    world: &mut World,
    direction: Direction,
    goal_tracker: &mut GoalTracker,
    item_type: Option<Content>,
) -> Result<usize, LibError> {
    match destroy(robot, world, direction) {
        Ok(removed_quantity) => {
            goal_tracker.update(Ok(()), GoalType::GetItems, item_type, removed_quantity);
            Ok(removed_quantity)
        }
        Err(err) => Err(err),
    }
}

fn handle_put(
    robot: &mut impl Runnable,
    world: &mut World,
    content_in: Content,
    quantity: usize,
    direction: Direction,
    goal_tracker: &mut GoalTracker,
    goal_type: GoalType,
) -> Result<usize, LibError> {
    match put(robot, world, content_in.clone(), quantity, direction) {
        Ok(removed_quantity) => {
            goal_tracker.update(Ok(()), goal_type, Some(content_in), removed_quantity);
            Ok(removed_quantity)
        }
        Err(err) => Err(err),
    }
}
