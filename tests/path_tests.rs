#[cfg(test)]
mod tests {
    use std::mem::discriminant;
    use std::time::Instant;

    use rayon::prelude::*;
    use robotics_lib::world::tile::{Content, Tile, TileType};

    use bob_lib::enhanced_map::BobPinTypes;

    /// Generates a String representation of a grid of tiles.
    /// Each tile is formatted and printed within a Markdown-like table structure.
    ///
    /// # Arguments
    ///
    /// * `tiles` - A vector of vectors representing the grid of tiles.
    ///
    pub fn pretty_print_tilemap(tiles: Vec<Vec<Tile>>) {
        for row in tiles {
            let mut row_str = String::new();
            for tile in row {
                let content_display = match tile.content {
                    Content::None => format!("{:?}", tile.tile_type),
                    _ => format!("{:?}({})", tile.tile_type, tile.content),
                };
                row_str += &format!("| {:<8}\t", content_display); // Adjust the width as needed
            }
            println!("{}|", row_str);
        }
    }

    pub fn match_content_type_variant(lhs: Option<Content>, rhs: Option<Content>) -> bool {
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => discriminant(&lhs) == discriminant(&rhs),
            _ => false,
        }
    }

    pub fn manual_update_testing(robot_map: &mut Vec<Vec<Option<robotics_lib::world::tile::TileType>>>, m: &mut Vec<Vec<(Option<robotics_lib::world::tile::TileType>, Option<BobPinTypes>)>>) {
        m.par_iter_mut().enumerate().for_each(|(i, v)| {
            v.iter_mut().enumerate().for_each(|(j, (tile, _))| {
                match robot_map[i][j] {
                    Some(val) => *tile = Some(val),
                    None => {}
                }
            })
        })
    }

    #[test]
    fn test_update_speed() {
        println!("time to create vectors: ");
        let start = Instant::now();
        let mut map = vec![vec![(Some(TileType::Street), Some(BobPinTypes::City)); 10000]; 10000];
        let mut robot_map = vec![vec![Some(robotics_lib::world::tile::TileType::Grass); 10000]; 10000];
        let end = Instant::now();
        let dur = end.duration_since(start);
        println!("{:?}", dur);

        println!("time to replace vectors: ");
        let start = Instant::now();
        manual_update_testing(&mut robot_map, &mut map);
        let end = Instant::now();
        let dur = end.duration_since(start);
        println!("{:?}", dur);
        assert_eq!(map[0][0].0, robot_map[0][0]);
    }

    /*
    #[test]
    fn test_pin() {
        let mut map = BobMap::init();
        let custom_pin = vec![5.6];
        let pin = BobPinTypes::Market;
        map.add_pin(Rc::new(pin), (0, 0));
        let res = map.get_pin((0, 0));
        matches!(BobPinTypes::Market, res);

        let pin = Rc::new(BobPinTypes::Custom(Rc::new(custom_pin)));
        map.add_pin(pin.clone(), (0, 1));
        let res = map.get_pin((0, 1)).unwrap();
        matches!(BobPinTypes::Custom, res);

        match res.deref() {
            BobPinTypes::Custom(v) => {
                if let Ok(v2) = bob_type_check::<Vec<f64>>(v.clone()) {
                    assert_eq!(5.6, v2[0])
                }
                if let Ok(v3) = bob_type_check::<i32>(v.clone()) {
                    assert_eq!(12, *v3)
                }
            }
            _ => {}
        }

        let found = map.search_pin(pin.clone());
        match found {
            Some(val) => {
                assert_eq!(val[0], (0, 1))
            }
            None => {}
        }
    }
     */

    #[test]
    fn test_match_content_type_both_some_same_content() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Rock(1));
        assert!(match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_some_same_content_different_value() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Rock(2));
        assert!(match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_some_different_content() {
        let lhs = Some(Content::Rock(1));
        let rhs = Some(Content::Tree(2));
        assert!(!match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_one_none() {
        let lhs = Some(Content::Rock(1));
        let rhs = None;
        assert!(!match_content_type_variant(lhs, rhs));
    }

    #[test]
    fn test_match_content_type_both_none() {
        let lhs: Option<Content> = None;
        let rhs: Option<Content> = None;
        assert!(!match_content_type_variant(lhs, rhs));
    }
}
