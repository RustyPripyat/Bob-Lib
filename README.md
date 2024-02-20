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

#### Manual Update Goals
In case the tool you've purchased independently calls the `put` and `destroy` interfaces, our tool provides a manual update for goals.
```rust
goal_tracker.update_manual(GoalType::GetItems, Some(Content::Rock), 3);
```
This method updates the first goal that matches the `GoalType` and `Content`. You don't need to check if the removed quantity is greater than the amount needed for the goal to complete.

## Enhanced Map

The enhanced map is one other functionality of our tool, it allows to have a richer robot map augmented with better coordinates tracking and customizable pins.

### Creating the Enhanced map

```rust
// initialize the enhanced map
let map = BobMap::init(&world);
```

### Using the Enhanced map

The BobMap can be used to create customizable pins and to ease your life when working with coordinates

#### Custom pins

There are various types of pins you can use, they are all predefined, but if you have some other pin type in mind
you can use the Custom pin which can contain whatever you like.
```rust
// all the pin types
pub enum BobPinTypes {
    I32(i32),
    String(String),
    TileType(TileType),
    Contents(Content),
    City,
    Bank(usize),
    Market,
    Custom(Arc<dyn Any + Send + Sync>),
}
```

then you can add them in the map on both discovered and undiscovered Tiles
```rust
// get the map
let map = BobMap::init(&world);

// add a pin
map.add_pin(BobPinTypes::Market, (3, 5));
```

Of course you can delete your pins...
```rust
let result = map.delete_pin((3, 5));
```

Search for them by coordinate...
```rust
let pin = map.get_pin((3, 5));
```

Or by Pin type
```rust
// returns every tile with this particular Pin type
let coordinates = map.search_pin(BobPinTypes::Market)
```

You can also get the whole map, you have to specify if it has been updated without the use of our interfaces
with a BobFlag
```rust
// if you used some other tool or interface and the map has been updated
let enhanced_map = map.get_map(&world, BobMapFlag::TilesUpdated);
// or if there has been no updates to the map outside the use of our view interfaces
let enhanced_map = map.get_map(&world, BobMapFlag::NoTileUpdated);
```
#### Absolute coordinates

our Enhanced map provides interfaces identical to the standard views of the robotic_lib with the added benefit
of returning the coordinates of what the robot sees.

```rust
// normal view around the robot but with absolute coordinates
let view = bob_view(&robot, &world, &mut map);

// long view as in one_direction_view but with absolute coordinates
let long_view = bob_one_direction_view(&mut robot, &world, Direction::Up, 3, &mut map);
```

#### Utility

Since using our Custom pin is not really Intuitive because of Any dyn, we provided a utility function to help
identifying the type of the Custom pin on the receiving end.
```rust
// returns Ok(<correct type value>) if the CustomType is correct
let result = bob_type_check::<CustomType>(Arc::clone(value));
```