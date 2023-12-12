## GoalTracker Usage Guide

The primary way to utilize the tracker provided by our tool is to use a `GoalTracker` struct and its methods.

### Creating GoalTracker

To start tracking goals, create a `GoalTracker` object using the provided `new` method.

```rust
let mut goal_tracker = GoalTracker::new();
```

### Adding Goals

Create `Goal` objects using the designated method and add them to the `GoalTracker` using the `add_goal` method.

```rust
// Create a goal
let new_goal = Goal::new(
"Get some fish".to_string(),
"Thanks for all the fish".to_string(),
GoalType::GetItems,
Some(Content::Fish),
42,
);

// Add the goal to the tracker
goal_tracker.add_goal(new_goal);
```

### Tracking Bot Actions

To track the actions of the robot, utilize specific methods provided by the `GoalTracker` for different actions:

#### Destroy and Collect Item

```rust
// Perform the action
destroy_and_collect_item(robot, world, direction, & mut goal_tracker);
```

This method internally calls the `destroy` interface. It then tries to update the goals that match the `GetItems`
goalType.

#### Throw Garbage

```rust
// Perform the action
throw_garbage(robot, world, content, quantity, direction, & mut goal_tracker);
```

This method internally calls the `put` interface. It then tries to update the goals that match the `ThrowGarbage`
goalType. It cheks if the robot is in front of a `Content::Bin`; the `content` must be of type `Content::Garbage`

#### Sell Items in Market

```rust
// Perform the action
sell_items_in_market(robot, world, content, quantity, direction, & mut goal_tracker);
```

This method internally calls the `put` interface. It then tries to update the goals that match the `SellItems` goalType.
It cheks if the robot is in front of a `Content::Market`.

#### Put Out Fire

```rust
// Perform the action
put_out_fire(robot, world, direction, & mut goal_tracker);
```

This method internally calls the `put` interface. It then tries to update the goals that match the `PutOutFire`
goalType. It checks if the robot is in front of a `Content::Fire`; the `content` must be of type `Content::Water`

Using these methods within the `GoalTracker` ensures a systematic way of managing and updating goals based on the
actions performed by the robot.

## Enhanced Map

The enhanced map is one other functionality of our tool, it allows to have a richer robot map augmented with better coordinates tracking and customizable pins.

### Custom pins

todo!

### Absolute coordinates

todo!

### Mandatory Interfaces

todo!

## Installation

Add this library as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
bob_lib = { git = "https://github.com/RustyPripyat/Bob-Lib.git", branch = "main" }
```
