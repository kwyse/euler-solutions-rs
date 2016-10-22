//! Tests for problems 11 through 20

#[test]
fn p011() {
    use mat::Direction;
    use ut::resource;

    let buffer: String = resource::from_file("p011").unwrap();
    let matrix = buffer.parse::<::Matrix<u32>>().unwrap();

    let directions = [Direction::Right, Direction::DownRight, Direction::Down, Direction::DownLeft];
    let mut products: Vec<u32> = Vec::new();

    for (index, item) in matrix.clone().enumerate() {
        for direction in &directions {
            if let Some(adjacent_indexes) = matrix.as_ref().adjacent_indexes(index, 3, direction) {
                if let Some(adjacent_items) = matrix.items_at_indexes(adjacent_indexes) {
                    products.push(adjacent_items.iter().product::<u32>() * item);
                }
            }
        }
    }

    assert_eq!(&70_600_674, products.iter().max().unwrap());
}
