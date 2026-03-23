use std::ops::{Add, Sub};

use super::*;

#[test]
fn coordinates_add_center_horizontal() {
    let result = Coordinates::new(5, 7).add(OrientationValue::Horizontal(1));
    assert_eq!(result, Some(Coordinates::new(5, 8)));
}

#[test]
fn coordinates_add_center_vertical() {
    let result = Coordinates::new(5, 7).add(OrientationValue::Vertical(1));
    assert_eq!(result, Some(Coordinates::new(6, 7)));
}

#[test]
fn coordinates_add_top_left_horizontal() {
    let result = Coordinates::new(0, 0).add(OrientationValue::Horizontal(1));
    assert_eq!(result, Some(Coordinates::new(0, 1)));
}

#[test]
fn coordinates_add_top_left_vertical() {
    let result = Coordinates::new(0, 0).add(OrientationValue::Vertical(1));
    assert_eq!(result, Some(Coordinates::new(1, 0)));
}

#[test]
fn coordinates_add_top_right_horizontal() {
    let result = Coordinates::new(0, BOARD_SIZE - 1).add(OrientationValue::Horizontal(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_top_right_vertical() {
    let result = Coordinates::new(0, BOARD_SIZE - 1).add(OrientationValue::Vertical(1));
    assert_eq!(result, Some(Coordinates::new(1, BOARD_SIZE - 1)));
}

#[test]
fn coordinates_add_bottom_left_horizontal() {
    let result = Coordinates::new(BOARD_SIZE - 1, 0).add(OrientationValue::Horizontal(1));
    assert_eq!(result, Some(Coordinates::new(BOARD_SIZE - 1, 1)));
}

#[test]
fn coordinates_add_bottom_left_vertical() {
    let result = Coordinates::new(BOARD_SIZE - 1, 0).add(OrientationValue::Vertical(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_horizontal() {
    let result =
        Coordinates::new(BOARD_SIZE - 1, BOARD_SIZE - 1).add(OrientationValue::Horizontal(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_vertical() {
    let result =
        Coordinates::new(BOARD_SIZE - 1, BOARD_SIZE - 1).add(OrientationValue::Vertical(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_center_horizontal() {
    let result = Coordinates::new(5, 7).sub(OrientationValue::Horizontal(1));
    assert_eq!(result, Some(Coordinates::new(5, 6)));
}

#[test]
fn coordinates_sub_center_vertical() {
    let result = Coordinates::new(5, 7).sub(OrientationValue::Vertical(1));
    assert_eq!(result, Some(Coordinates::new(4, 7)));
}

#[test]
fn coordinates_sub_top_left_horizontal() {
    let result = Coordinates::new(0, 0).sub(OrientationValue::Horizontal(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_left_vertical() {
    let result = Coordinates::new(0, 0).sub(OrientationValue::Vertical(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_right_horizontal() {
    let result = Coordinates::new(0, BOARD_SIZE - 1).sub(OrientationValue::Horizontal(1));
    assert_eq!(result, Some(Coordinates::new(0, BOARD_SIZE - 2)));
}

#[test]
fn coordinates_sub_top_right_vertical() {
    let result = Coordinates::new(0, BOARD_SIZE - 1).sub(OrientationValue::Vertical(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_horizontal() {
    let result = Coordinates::new(BOARD_SIZE - 1, 0).sub(OrientationValue::Horizontal(1));
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_vertical() {
    let result = Coordinates::new(BOARD_SIZE - 1, 0).sub(OrientationValue::Vertical(1));
    assert_eq!(result, Some(Coordinates::new(BOARD_SIZE - 2, 0)));
}

#[test]
fn coordinates_sub_bottom_right_horizontal() {
    let result =
        Coordinates::new(BOARD_SIZE - 1, BOARD_SIZE - 1).sub(OrientationValue::Horizontal(1));
    assert_eq!(
        result,
        Some(Coordinates::new(BOARD_SIZE - 1, BOARD_SIZE - 2)),
    );
}

#[test]
fn coordinates_sub_bottom_right_vertical() {
    let result =
        Coordinates::new(BOARD_SIZE - 1, BOARD_SIZE - 1).sub(OrientationValue::Vertical(1));
    assert_eq!(
        result,
        Some(Coordinates::new(BOARD_SIZE - 2, BOARD_SIZE - 1)),
    );
}
