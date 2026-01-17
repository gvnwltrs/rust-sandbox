
// Struct example case
#[derive(Debug)]
#[allow(unused)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[test]
fn test_rectangle_set_width_height() {
    let rectangle = Rectangle::new(2, 5);
    assert_eq!(rectangle.width, 2);
    assert_eq!(rectangle.height, 5);
}
