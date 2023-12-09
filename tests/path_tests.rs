#[path = "../src/bob.rs"]
mod bob;

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::bob::{bob_get_pin, bob_pin, bob_type_check, BobMap, BobPinTypes};

    #[test]
    fn test_pin(){
        let mut map = BobMap::init();
        let custom_pin = vec![5.6];
        let pin = BobPinTypes::Market;
        bob_pin(&mut map, pin);
        let res = bob_get_pin(&mut map);
        matches!(BobPinTypes::Market, res);

        let pin = BobPinTypes::Custom1(Rc::new(custom_pin));
        bob_pin(&mut map, pin);
        let res = bob_get_pin(&mut map);
        matches!(BobPinTypes::Custom1, res);

        match res {
            BobPinTypes::Custom1(v) => {
                if let Ok(v2) = bob_type_check::<Vec<f64>>(v.clone()){
                    assert_eq!(5.6, v2[0])
                }
                if let Ok(v3) = bob_type_check::<i32>(v.clone()){
                    assert_eq!(12, *v3)
                }
            },
            _ => {}
        }
    }
}