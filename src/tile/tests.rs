use super::*;

#[test]
fn tile_iterator() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('F', Coordinates::new(3, 2), false),
        Tile::new('U', Coordinates::new(3, 3), false),
        Tile::new('C', Coordinates::new(3, 4), false),
        Tile::new('K', Coordinates::new(3, 5), false),
        Tile::new('Y', Coordinates::new(4, 0), false),
        Tile::new('O', Coordinates::new(4, 1), false),
        Tile::new('U', Coordinates::new(4, 2), false),
        Tile::new('N', Coordinates::new(5, 2), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let mut tile_iter = TileIterator::new(&board);

    for tile in tiles {
        assert_eq!(tile_iter.next(), Some(tile).as_ref());
    }

    assert_eq!(tile_iter.next(), None);
}
