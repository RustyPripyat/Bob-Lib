use std::fmt;
use std::fmt::Display;

use crate::utils::{get_tile_in_direction, match_content_type_variant};
use robotics_lib::interface::{destroy, put, Direction};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;
use robotics_lib::world::World;

/// Represents various types of goals in a robotics context.
///
/// # Variants
///
/// * `PutOutFire` - Represents a goal to extinguish a fire.
/// * `GetItems` - Represents a goal to retrieve items.
/// * `SellItems` - Represents a goal to sell items.
/// * `ThrowGarbage` - Represents a goal to dispose of garbage.
#[derive(Debug, PartialEq)]
pub enum GoalType {
    PutOutFire,
    GetItems,
    SellItems,
    ThrowGarbage,
}

/// Represents a goal in a robotics context.
///
/// # Arguments
///
/// * `name` - The name of the goal.
/// * `description` - A description providing details about the goal.
/// * `goal_type` - The type of goal (e.g., PutOutFire, GetItems, SellItems, ThrowGarbage).
/// * `item_type` - The optional type of content associated with the goal (e.g., Some(Content)).
/// * `completed` - Indicates whether the goal has been completed (true) or not (false).
/// * `goal_quantity` - The quantity required to fulfill the goal.
/// * `items_left` - The number of items left to complete the goal.
#[derive(Debug)]
pub struct Goal {
    /// The name of the goal.
    pub name: String,

    /// A description providing details about the goal.
    pub description: String,

    /// The type of goal.
    pub goal_type: GoalType,

    /// The optional type of content associated with the goal.
    pub item_type: Option<Content>,

    /// Indicates whether the goal has been completed.
    pub completed: bool,

    /// The quantity required to fulfill the goal.
    pub goal_quantity: u32,

    /// The number of items left to complete the goal.
    pub items_left: u32,
}

impl Goal {
    /// Creates a new Goal instance.
    ///
    /// # Arguments
    /// * `name` - The name of the goal.
    /// * `description` - The description of the goal.
    /// * `goal_type` - The type of the goal.
    /// * `item_type` - The optional type of the item associated with the goal.
    /// * `goal_quantity` - The quantity related to the goal.
    ///
    /// # Returns
    /// A new `Goal` instance.
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

    /// Updates the goal progress based on the removed quantity.
    fn update_progress(&mut self, removed_quantity: usize) {
        self.items_left = self.items_left.saturating_sub(removed_quantity as u32);
        if self.items_left == 0 {
            self.completed = true; // Imposta il goal come completato se non ci sono più elementi rimasti.
        }
    }

    fn is_completed(&self) -> bool {
        self.items_left == 0
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

/// Tracks and manages goals within a robotics context.
///
/// # Arguments
///
/// * `goals` - A vector storing the list of goals to be tracked.
/// * `completed_number` - The count of completed goals within the tracker.
pub struct GoalTracker {
    /// The list of goals being tracked.
    goals: Vec<Goal>,

    /// The number of completed goals.
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

    /// Finds a goal an returs a mut ref to such goal. Finds the first occurrency.
    fn find_goal_mut(
        &mut self,
        goal_type: GoalType,
        item_type: Option<Content>,
    ) -> Option<&mut Goal> {
        self.goals.iter_mut().find(|goal| {
            goal.goal_type == goal_type
                && match_content_type_variant(goal.item_type.clone(), item_type.clone())
        })
    }

    // Removes all completed goals from the tracker.
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

    /// Manually update a goal's progress based on specified parameters.
    ///
    /// This method allows for manual tracking of goal progress by specifying the goal type,
    /// associated item type, and the quantity of items removed or completed. It searches for
    /// the corresponding goal in the tracker and updates its status accordingly.
    ///
    /// # Arguments
    ///
    /// * `goal_type` - The type of the goal to be updated.
    /// * `item_type` - The optional type of content associated with the goal.
    /// * `removed_quantity` - The quantity of items removed or completed.
    ///
    /// # Examples
    /// ```
    /// use bob_lib::{GoalTracker, GoalType};
    ///
    /// let mut goal_tracker = GoalTracker::new();
    /// // Assume a goal of type GetItems with some associated content
    /// goal_tracker.update_manual(GoalType::GetItems, Some(Content::Rock), 3);
    /// ```
    ///
    /// This example demonstrates manually updating a goal of type `GetItems` with the removal of
    /// 3 items of type `Content::Rock`, in case the goal was not updated automatically, because an
    /// external tool(s) called the `put` or the `destroy` interface directly.
    ///
    /// # Note
    ///
    /// If the goal is not found in the tracker, an error message is printed to the standard error.
    ///
    /// # Panics
    ///
    /// The method does not panic but may print an error message if the specified goal is not found.
    ///
    /// # Safety
    ///
    /// This method assumes that the `GoalTracker` is correctly initialized and that the provided
    /// parameters are valid in the context of the tracked goals.
    pub fn update_manual(
        &mut self,
        goal_type: GoalType,
        item_type: Option<Content>,
        removed_quantity: usize,
    ) {
        if let Some(goal) = self.find_goal_mut(goal_type, item_type.clone()) {
            println!("Found goal: {:?}", goal);
            goal.update_progress(removed_quantity);
            if goal.is_completed() {
                self.completed_number += 1;
            }
        } else {
            eprintln!("Error: Goal not found");
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
/// One unit of `Content::Water` from the backpack is used to put out the fire.
///
/// # Arguments
/// * `robot` - The robot that will perform the action.
/// * `world` - The world in which the action takes place.
/// * `direction` - The direction in which to perform the action.
/// * `goal_tracker` - The goal tracker to update upon successfully putting out the fire.
///
/// # Returns
/// Result<(usize), LibError> - Ok((removed_quantity)) if the action is successful, Err(LibError) otherwise.
///
pub fn put_out_fire(
    robot: &mut impl Runnable,
    world: &mut World,
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
        Content::Water(0),
        1,
        direction,
        goal_tracker,
        GoalType::PutOutFire,
    )
    // Ok(())
}

/// Sells items in a specified direction by using the robot to perform the action.
/// It automatically checks if the robot is in front of a market and if the content to sell is
/// valid. If not, it returns an error. It does update all your goals if the action is successful.
/// It calls the put interface internally from Robotics_lib.
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
/// It calls the put interface internally from Robotics_lib.
///
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
/// It calls the destroy interface internally from Robotics_lib.
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
