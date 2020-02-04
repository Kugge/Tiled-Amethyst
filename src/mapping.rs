mod parsing;

struct Properties {

}

struct Tile {
    height: i32,  // Pixels
    width: i32,   // Pixels
    properties: Properties
}

struct Tileset {
    source: String,  // Path to file
    height: i32,     // Tiles
    width: i32,      // Tiles
    name: String,
    tiles: Vec<Tile>
}

pub struct Untiled_Layer {  // Used for parsing
    id: usize,
    name: String,
    width: u32,
    height: u32,
    matrix: Vec<Vec<i32>>
}

pub struct Layer {  // Used for parsing
    id: usize,
    name: String,
    width: u32,
    height: u32,
    matrix: Vec<Tile>
}

pub struct Map {
    source: String,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>
}

pub fn build_map(f: &str) -> Map {
    Map{
        source: f.to_string(),
        layers: get_layers(f)
    }
}

fn main() {
    let map: Map = build_map(&"test.tmx");
    println!("{:?}", map)
}
