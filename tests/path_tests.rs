#[path = "../src/bob.rs"]
mod bob;

#[cfg(test)]
mod tests {
    use robotics_lib::interface::Direction::*;
    use robotics_lib::world::World;

    use crate::bob::navigate_rectangular;

    #[test]
    fn test_navigate_rectangular_right() {
        let start = (1, 1);
        let end = (1, 4);
        let path = navigate_rectangular(start, end);
        assert_eq!(path, vec![Right, Right, Right]);
    }

    #[test]
    fn test_navigate_rectangular_left() {
        let start = (1, 4);
        let end = (1, 1);
        let path = navigate_rectangular(start, end);
        assert_eq!(path, vec![Left, Left, Left]);
    }

    #[test]
    fn test_navigate_rectangular_up() {
        let start = (4, 1);
        let end = (1, 1);
        let path = navigate_rectangular(start, end);
        assert_eq!(path, vec![Down, Down, Down]);
    }

    #[test]
    fn test_navigate_rectangular_down() {
        let start = (1, 1);
        let end = (4, 1);
        let path = navigate_rectangular(start, end);
        assert_eq!(path, vec![Up, Up, Up]);
    }

    #[test]
    fn test_navigate_rectangular_diagonal() {
        let start = (1, 1);
        let end = (4, 5);
        let path = navigate_rectangular(start, end);
        // println!("{:?}", path);
        assert_eq!(path, vec![Up, Up, Up, Right, Right, Right, Right]);
    }

    #[test]
    fn test_navigate_rectangular_diagonal_backwards() {
        let start = (6, 8);
        let end = (1, 2);
        let path = navigate_rectangular(start, end);
        // println!("{:?}", path);
        assert_eq!(
            path,
            vec![Down, Down, Down, Down, Down, Left, Left, Left, Left, Left, Left]
        );
    }

    #[test]
    fn test_navigate_rectangular_no_movement_same_points() {
        let start = (5, 5);
        let end = (5, 5);
        let path = navigate_rectangular(start, end);
        assert_eq!(path, Vec::new());
    }
}
