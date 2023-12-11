# Goal Tracker Rust Library

This Rust library provides functionality for tracking and managing goals within a robotics context. It includes methods to create, manage, update, and remove various types of goals that a robot needs to accomplish.

## Purpose

The `GoalTracker` module manages different types of goals, such as putting out fires, obtaining items, selling items, and disposing of garbage. It allows for easy tracking of the status of these goals and their completion.

## Features

### `Goal` Struct
- Represents individual goals with details like name, description, type, completion status, goal quantity, and remaining items.
- Goals can be created, updated, and removed.

### `GoalTracker` Struct
- Manages a collection of goals.
- Tracks completed goals and updates their status.
- Allows for adding, removing, and retrieving goals.

### Goal Types
- **PutOutFire**: Puts out fires in a specified direction.
- **GetItems**: Obtains items in a specified direction.
- **SellItems**: Sells items in a specified direction.
- **ThrowGarbage**: Disposes of garbage in a specified direction.

### Functions
All these methods are basically wrappers around the functions provided by [Robotics_lib](https://github.com/Advanced-Programming-2023/Robotic-Lib).
The library includes functions to perform actions related to these goals:
- `put_out_fire`: Puts out fires using a robot in a specified direction.
- `get_items`: Retrieves items using a robot in a specified direction.
- `sell_items`: Sells items using a robot in a specified direction.
- `throw_garbage`: Disposes of garbage using a robot in a specified direction.

## Usage
To use this library:
- Integrate the `GoalTracker` and related methods into your Rust project.
- Define goals using the `Goal` struct and add them to the `GoalTracker`.
- Use the provided functions (`put_out_fire`, `get_items`, `sell_items`, `throw_garbage`) to perform actions instead of directly calling the interfaces provided by [Robotics_lib](https://github.com/Advanced-Programming-2023/Robotic-Lib) so that all your goals stay update.

## Installation

Add this library as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
goal_tracker = { git = "https://github.com/RustyPripyat/Bob-the-poor-sheikah-clone.git", branch = "main" }
```