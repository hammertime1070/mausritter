use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
}

// Function that returns a vector of tiles
// Use a vector sized to the whole map
// This function figures out which array index is at a given x/y position
// 80 is the map width
pub fn xy_idx(x: 132, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

// Map constructor
fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80*50];

    // Make the boundary walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now a bunch of random walls
    for _i in 0..400 {
        let x = rng.gen_range(1..79 as i32)
        let y = rng.gen_range(1..49 as i32)
        let idx = xy_idx(40, 25);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }
    return map;
}

