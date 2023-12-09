#[path = "../src/bob.rs"]
mod bob;

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::rc::Rc;
    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::World;
    use crate::bob::{bob_type_check, BobMap, BobPinTypes};


    #[test]
    fn test_pin(){
        let mut map = BobMap::init();
        let custom_pin = vec![5.6];
        let pin = BobPinTypes::Market;
        map.add_pin(Rc::new(pin), (0, 0));
        let res = map.get_pin((0,0));
        matches!(BobPinTypes::Market, res);

        let pin = Rc::new(BobPinTypes::Custom(Rc::new(custom_pin)));
        map.add_pin(pin.clone(), (0, 1));
        let res = map.get_pin((0, 1)).unwrap();
        matches!(BobPinTypes::Custom, res);

        match res.deref() {
            BobPinTypes::Custom(v) => {
                if let Ok(v2) = bob_type_check::<Vec<f64>>(v.clone()){
                    assert_eq!(5.6, v2[0])
                }
                if let Ok(v3) = bob_type_check::<i32>(v.clone()){
                    assert_eq!(12, *v3)
                }
            },
            _ => {}
        }

        let found = map.search_pin(pin.clone());
        match found {
            Some(val) => {
                assert_eq!(val[0], (0,1))
            },
            None => {}
        }
    }

}