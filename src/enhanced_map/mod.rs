use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

use rayon::prelude::*;
use robotics_lib::interface::{Direction, discover_tiles, one_direction_view, robot_map, robot_view};
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::World;

/// Enum that contains every possible pin type
/// # Arguments
/// * `I32(i32)`
/// * `String(String)`
/// * `TileType(TileType)`
/// * `Contents(Contents)`
/// * `City`
/// * `Bank(usize)`
/// * `Market`
/// * [`Custom(Arc<dyn Any>)`](BobPinTypes::Custom)
/// # Examples
/// ```
/// use robotics_lib::world::tile::Content;
/// use bob_lib::enhanced_map::BobPinTypes;
/// let pin = BobPinTypes::Contents(Content::Fish(5));
///
/// match pin {
///     BobPinTypes::Contents(content) => assert_eq!(content, Content::Fish(5)),
///     _ => todo!()
/// }
/// ```
#[derive(Debug, Clone)]
pub enum BobPinTypes {
    I32(i32),
    String(String),
    TileType(TileType),
    Contents(Content),
    City,
    Bank(usize),
    Market,
    /// Custom pin type
    ///
    /// Contains an Arc<dyn Any> meaning that it can contain
    /// any type you want as a pin.
    ///
    /// When obtained back the type is undefinable, the usage of
    /// [bob_type_check] is suggested.
    Custom(Arc<dyn Any + Send + Sync>),
}

/// ### Enum which says if the map has been updated
/// it is used in the function [get_map](BobMap::get_map) to say
/// if the map changed, if it didn't, the map won't try to auto_update
/// saving time
#[derive(PartialEq)]
pub enum BobMapFlag{
    NoTileUpdated,
    TilesUpdated
}

/// enum that contains some specific errors
pub enum BobErr{
    PinAlreadySet,
    PinNotFound,
    EmptyTile
}

impl PartialEq<Self> for BobPinTypes {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BobPinTypes::Market, BobPinTypes::Market) => true,
            (BobPinTypes::City, BobPinTypes::City) => true,
            (BobPinTypes::I32(val1), BobPinTypes::I32(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::String(val1), BobPinTypes::String(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::TileType(val1), BobPinTypes::TileType(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Contents(val1), BobPinTypes::Contents(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Bank(val1), BobPinTypes::Bank(val2)) => {
                if val1.eq(val2) {
                    return true;
                }
                false
            }
            (BobPinTypes::Custom(val1), BobPinTypes::Custom(val2)) => {
                if Arc::<(dyn Any + Send + Sync + 'static)>::as_ptr(val1)
                    .eq(&Arc::<(dyn Any + Send + Sync + 'static)>::as_ptr(val2))
                {
                    return true;
                }
                false
            }
            (_, _) => false,
        }
    }
}

impl Eq for BobPinTypes {}

impl Hash for BobPinTypes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match &self {
            BobPinTypes::I32(value) => value.hash(state),
            BobPinTypes::String(value) => value.hash(state),
            BobPinTypes::TileType(value) => value.hash(state),
            BobPinTypes::Contents(value) => value.hash(state),
            BobPinTypes::City => (),
            BobPinTypes::Bank(value) => value.hash(state),
            BobPinTypes::Market => (),
            BobPinTypes::Custom(Arc_any) => {
                let ptr = Arc::<(dyn Any + Send + Sync + 'static)>::as_ptr(&Arc_any);
                ptr.hash(state)
            }
        }
    }
}

/// Enhanced map containing Tiles + Pins
/// # Details
/// Regarding pins the map will always be updated
///
/// Regarding tiles, the map will be updated quickly if using
/// the interfaces [bob_view] and [bob_one_direction_view], and a bit more
/// slowly when it auto updates from calling [get_map](BobMap::get_map) with
/// the [BobMapFlag::TilesUpdated] flag
/// # Functionalities
/// * [`init`](BobMap::init): initialize map
/// * [`add_pin`](BobMap::add_pin): add a pin
/// * [`get_pin`](BobMap::get_pin): get a pin from coordinates
/// * [`get_map`](BobMap::get_map): get the map with pins
/// * [`delete_pin`](BobMap::delete_pin): delete a pin at coordinates
/// * [`search_pin`](BobMap::search_pin): search a pin from pin
pub struct BobMap {
    map: Vec<Vec<(Option<Tile>, Option<Arc<BobPinTypes>>)>>,
    pins_location: HashMap<Arc<BobPinTypes>, Vec<(usize, usize)>>,
}

impl BobMap {
    /// Init function to initialize the map
    ///
    /// Every time it is called it will return a new map with Tiles
    /// filled by the **current discovered map** and **no pins**
    /// # Example
    /// ```
    /// use robotics_lib::world::World;
    /// use bob_lib::enhanced_map::BobMap;
    ///
    /// let world: World;
    /// let mut map = BobMap::init(&world);
    /// ```
    pub fn init(world: &World) -> BobMap {
        let robot_map = robot_map(world).unwrap();
        let map: Vec<Vec<(Option<Tile>, Option<Arc<BobPinTypes>>)>> = robot_map
            .into_par_iter()
            .map(|row| row.into_iter().map(|tile| (tile, None)).collect())
            .collect();
        BobMap {
            map,
            pins_location: HashMap::new()
        }
    }

    fn update(&mut self, coordinates: Vec<(usize, usize, Tile)>) {
        for (x, y, tile) in coordinates {
            self.map[x][y].0 = Some(tile);
        }
    }

    fn auto_update(&mut self, world: &World) {
        let mut robot_map = robot_map(world).unwrap();
        let mut m = self.get_mut_map();

        m.par_iter_mut().enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, (tile, _))| {
                if robot_map[i][j].is_some() {
                    *tile = robot_map[i][j].clone()
                }
            })
        })
    }

    /// Function to add a pin to a location on the map
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use robotics_lib::world;
    /// use bob_lib::enhanced_map::{BobMap, BobPinTypes};
    ///
    /// let mut map: BobMap;
    /// map.add_pin(BobPinTypes::City, (1,3)).ok().unwrap()
    /// ```
    pub fn add_pin(&mut self, pin: BobPinTypes, (x, y): (usize, usize)) -> Result<(), BobErr>{
        if self.map[x][y].1.is_some() {
            return Err(BobErr::PinAlreadySet)
        }
        let arc_pin = Arc::new(pin);
        self.map[x][y].1 = Some(arc_pin.clone());
        if self.pins_location.contains_key(&arc_pin) {
            let vec = self.pins_location.get_mut(&arc_pin).unwrap();
            vec.push((x, y));
        } else {
            self.pins_location.insert(arc_pin.clone(), vec![(x, y)]);
        }
        Ok(())
    }

    /// Function to retrieve a pin from a location on the map
    ///
    /// It returns [None] if there are no pins at the coordinates,
    ///
    /// It returns [Some] containing a pointer to a [BobPinTypes] otherwise
    /// # Example
    /// ```
    /// use bob_lib::enhanced_map::BobMap;
    ///
    /// let mut map: BobMap;
    /// let result = map.get_pin((1, 3));
    /// ```
    pub fn get_pin(&self, (x, y): (usize, usize)) -> Option<Arc<BobPinTypes>> {
        self.map[x][y].1.clone()
    }

    /// Function to delete a pin from a location on the map
    ///
    /// It returns [Err] containing [BobErr::EmptyTile] if there are no pins at the coordinates
    ///
    /// It returns an empty [Ok] if the deletion was successful
    /// # Example
    /// ```
    /// use bob_lib::enhanced_map::BobMap;
    ///
    /// let mut map: BobMap;
    /// match map.delete_pin((1,3)) {
    ///     Ok(_) => println!("deleted"),
    ///     Err(_) => println!("nothing to delete")
    /// }
    /// ```
    pub fn delete_pin(&mut self, (x, y): (usize, usize)) -> Result<(), BobErr> {
        if self.map[x][y].1.is_some() {
            self.map[x][y].1 = None;
            return Ok(());
        }
        Err(BobErr::EmptyTile)
    }

    /// Function to get a full map with pins
    ///
    /// It returns a matrix of undiscovered and discovered Tiles, each associated with
    /// their pins
    ///
    /// If the map was updated by means different from our interfaces, the map will auto update
    /// taking more time
    /// # Example
    /// if the map was only updated through [bob_view], [bob_one_direction_view]. [add_pin](BobMap::add_pin), [bob_discover_tile] or it wasn't updated at all
    /// ```
    /// use robotics_lib::world::World;
    /// use bob_lib::enhanced_map::{BobMap, BobMapFlag};
    ///
    /// let mut map: BobMap;
    /// let world: World;
    /// let enhanced_map = map.get_map(&world, BobMapFlag::NoTileUpdated);
    /// ```
    /// if the map was updated by different means
    /// ```
    /// use robotics_lib::world::World;
    /// use bob_lib::enhanced_map::{BobMap, BobMapFlag};
    ///
    /// let mut map: BobMap;
    /// let world: World;
    /// let enhanced_map = map.get_map(&world, BobMapFlag::TilesUpdated);
    /// ```
    pub fn get_map(&mut self, world: &World, flag: BobMapFlag) -> &Vec<Vec<(Option<Tile>, Option<Arc<BobPinTypes>>)>> {
        if flag == BobMapFlag::TilesUpdated {
            self.auto_update(world);
        }
        self.map.as_ref()
    }

    fn get_mut_map(&mut self) -> &mut Vec<Vec<(Option<Tile>, Option<Arc<BobPinTypes>>)>> {
        self.map.as_mut()
    }

    /// Function to search a pin in the map
    ///
    /// It return [None] if the pin searched has not been placed
    ///
    /// It returns [Some] Containing a Vec of coordinates which all contain the specified
    /// pin
    /// # Example
    /// ```
    /// use std::sync::Arc;
    /// use bob_lib::enhanced_map::{BobMap, BobPinTypes};
    ///
    /// let map: BobMap;
    /// let coordinates = map.search_pin(BobPinTypes::Market);
    /// ```
    /// This function will keep in mind the value assigned to the enum, for example:
    ///
    /// ```
    /// use std::sync::Arc;
    /// use bob_lib::enhanced_map::{BobMap, BobPinTypes};
    ///
    /// let map: BobMap;
    /// // these two will return different coordinates
    /// let coordinates_1 = map.seaArch_pin(Arc::new(BobPinTypes::I32(5)));
    /// let coordinates_2 = map.seaArch_pin(Arc::new(BobPinTypes::I32(12)));
    /// ```
    pub fn search_pin(&self, pin: BobPinTypes) -> Result<Vec<(usize, usize)>, BobErr> {
        if self.pins_location.contains_key(&pin) {
            let vec = self.pins_location.get(&pin).unwrap();
            return Ok(vec.clone());
        } else {
            Err(BobErr::PinNotFound)
        }
    }
}

/// Function to replace the interface [robot_view]
///
/// It return a matrix 3x3 around the robot, containing the discovered tiles and the
/// absolute coordinates relative to the map
/// # Example
/// ```
/// use robotics_lib::runner::Robot;
/// use robotics_lib::world::World;
/// use bob_lib::enhanced_map::{bob_view, BobMap};
///
/// let world: World;
/// let robot: Robot;
/// let mut map: BobMap;
///
/// let view = bob_view(&robot, &world, &mut map);
/// ```
pub fn bob_view(
    robot: &impl Runnable,
    world: &World,
    map: &mut BobMap,
) -> Vec<Vec<(Option<Tile>, usize, usize)>> {
    let view = robot_view(robot, world);
    let pos = robot.get_coordinate();
    let mut update_vector: Vec<(usize, usize, Tile)> = vec![];
    let mut ret: Vec<Vec<(Option<Tile>, usize, usize)>> = vec![];

    for (i, v) in view.iter().enumerate() {
        ret.push(vec![]);
        for (j, tile) in v.iter().enumerate() {
            let x = pos.get_row() - 1 + i;
            let y = pos.get_col() - 1 + j;
            if tile.is_some() {
                update_vector.push((x, y, tile.clone().unwrap()));
                ret[i].push((tile.clone(), x, y));
            } else {
                ret[i].push((tile.clone(), x, y));
            }
        }
    }

    map.update(update_vector);
    ret
}

/// Function to replace the interface [one_direction_view]
///
/// It returns an [Err] containing a [LibErr] if it fails
///
/// It returns [Ok] containing a matrix of discovered Tiles and
/// their absolute positions relative to the map
///
/// # Example
/// ```
/// use robotics_lib::interface::Direction;
/// use robotics_lib::runner::Robot;
/// use robotics_lib::world::World;
/// use bob_lib::enhanced_map::{bob_one_direction_view, BobMap};
///
/// let mut map: BobMap;
/// let world: World;
/// let mut robot: Robot;
///
/// let view = bob_one_direction_view(&mut robot, &world, Direction::Up, 3, &mut map);
/// ```
pub fn bob_one_direction_view(
    robot: &mut impl Runnable,
    world: &World,
    direction: Direction,
    distance: usize,
    map: &mut BobMap,
) -> Result<Vec<Vec<(Tile, usize, usize)>>, LibError> {
    let long_view = one_direction_view(robot, world, direction.clone(), distance)?;
    let mut update_vector: Vec<(usize, usize, Tile)> = vec![];
    let mut ret: Vec<Vec<(Tile, usize, usize)>> = vec![];
    let pos = robot.get_coordinate();

    match direction {
        Direction::Up => {
            for (i, v) in long_view.iter().enumerate() {
                ret.push(vec![]);
                for (j, tile) in v.iter().enumerate() {
                    let x = pos.get_row() - 1 - i;
                    let y;
                    if pos.get_col() == 0 {
                        y = pos.get_col() + j;
                    } else {
                        y = pos.get_col() - 1 + j;
                    }

                    update_vector.push((x, y, tile.clone()));
                    ret[i].push((tile.clone(), x, y));
                }
            }
        }
        Direction::Down => {
            for (i, v) in long_view.iter().enumerate() {
                ret.push(vec![]);
                for (j, tile) in v.iter().enumerate() {
                    let x = pos.get_row() + 1 + i;
                    let y;
                    if pos.get_col() == 0 {
                        y = pos.get_col() + j;
                    } else {
                        y = pos.get_col() - 1 + j;
                    }

                    update_vector.push((x, y, tile.clone()));
                    ret[i].push((tile.clone(), x, y));
                }
            }
        }
        Direction::Left => {
            for (i, v) in long_view.iter().enumerate() {
                ret.push(vec![]);
                for (j, tile) in v.iter().enumerate() {
                    let y = pos.get_col() - 1 - j;
                    let x;
                    if pos.get_row() == 0 {
                        x = pos.get_row() + i;
                    } else {
                        x = pos.get_row() - 1 + i;
                    }
                    update_vector.push((x, y, tile.clone()));
                    ret[i].push((tile.clone(), x, y));
                }
            }
        }
        Direction::Right => {
            for (i, v) in long_view.iter().enumerate() {
                ret.push(vec![]);
                for (j, tile) in v.iter().enumerate() {
                    let y = pos.get_col() + 1 + j;
                    let x;
                    if pos.get_row() == 0 {
                        x = pos.get_row() + i;
                    } else {
                        x = pos.get_row() - 1 + i;
                    }
                    update_vector.push((x, y, tile.clone()));
                    ret[i].push((tile.clone(), x, y));
                }
            }
        }
    }

    map.update(update_vector);
    Ok(ret)
}

/// Discovers tiles in the world based on specified coordinates and updates the BobMap.
///
/// # Arguments
///
/// * `robot` - A mutable reference to a robot implementing the `Runnable` trait.
/// * `world` - A mutable reference to the world in which the robot operates.
/// * `to_discover` - A slice containing coordinates `(usize, usize)` of tiles to discover.
/// * `map` - A mutable reference to the BobMap that needs to be updated.
///
/// # Returns
///
/// Returns a Result containing a HashMap<(usize, usize), Option<Tile>> where:
/// - The key is a tuple representing coordinates.
/// - The value is an Option that may contain a Tile representing discovered information.
/// - Possible errors are wrapped in a LibError.
///
/// # Errors
///
/// This function may return an error if there are issues during the tile discovery process.
///
/// ```
pub fn bob_discover_tiles(robot: &mut impl Runnable,
                          world: &mut World,
                          to_discover: &[(usize, usize)],
                          map: &mut BobMap) -> Result<HashMap<(usize, usize), Option<Tile>>, LibError> {
    let discover_result = discover_tiles(robot, world, to_discover);
    let mut discovered_tiles_vec: Vec<(usize, usize, Tile)> = vec![];

    // we get the hashmap and turn it into a simpler Vec<(usize,usize,Tile)>
    // so we can feed such vec to the BobMap::update() function

    match discover_result.clone() {
        Ok(hashmap) => {
            discovered_tiles_vec = hashmap
                .into_iter()
                .filter_map(|((x, y), tile_option)| {
                    tile_option.map(|tile| (x, y, tile))
                })
                .collect();
        }
        Err(e) => return Err(e)
    }

    // println!("discovered tiles: {:?}", discovered_tiles_vec);

    map.update(discovered_tiles_vec);
    discover_result
}

/// Function to check the type of a [BobPinTypes::Custom] after receiving it back
///
/// It returns an empty [Err] if the Custom is not of the requested type
///
/// It return [Ok] containing an [Arc] pointing to a value of the requested Type
/// if the requested type is indeed the correct one
/// # Example
/// ```
/// use std::ops::Deref;
/// use std::rc::Rc;
/// use std::sync::Arc;
/// use bob_lib::enhanced_map::{bob_type_check, BobMap, BobPinTypes};
///
/// // custom pin type
/// enum CustomPin{
///     Whatever
/// }
///
/// let mut map: BobMap;
/// // add pin to coordinates (0,0)
/// map.add_pin(BobPinTypes::Custom(Arc::new(CustomPin::Whatever)), (0,0));
/// // assume it returns Some for semplicity
/// let pin = map.get_pin((0,0)).unwrap().deref();
///
/// match pin {
///     BobPinTypes::Custom(value) => {
///         // if value is of type CustomPin returns Ok(value)
///         if let Ok(found) = bob_type_check::<CustomPin>(Arc::clone(value)) {
///             matches!(found.deref(), CustomPin::Whatever);
///         }
///     }
///     _ => todo!()
/// }
/// ```
pub fn bob_type_check<T: Send + Sync + 'static>(to_check: Arc<dyn Any + Send + Sync>) -> Result<Arc<T>, ()> {
    if let Some(val) = to_check.downcast::<T>().ok() {
        return Ok(val);
    }
    Err(())
}