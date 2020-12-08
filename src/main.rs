mod types;

fn main() {
    let tile_1 = types::Tile::new_numbered(types::NumberedSuit::Characters, 4, true);
    let tile_2 = types::Tile::new_numbered(types::NumberedSuit::Circles, 7, false);
    let tile_3 = types::Tile::new_numbered(types::NumberedSuit::Bamboos, 2, false);
    let tile_4 = types::Tile::new_dragon(types::DragonColor::Red);
    let tile_5 = types::Tile::new_wind(types::WindDirection::South);

    println!("{},{},{},{},{}", tile_1, tile_2, tile_3, tile_4, tile_5);
}
