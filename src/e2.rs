pub const E2_COLUMNS: usize = 16;
pub const E2_ROWS: usize = 16;
pub const E2_TILE_COUNT: usize = E2_COLUMNS * E2_ROWS;

pub type E2_Board = crate::board::Board<Edge, E2_COLUMNS, E2_ROWS>;
pub type E2_Tile = crate::board::Tile<Edge>;
pub type E2_Tiles = crate::board::Tiles<Edge, E2_TILE_COUNT>;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
/// A rust edge is either *outside* (grey) or one of the 22 two-color patterns.
/// 
/// 
pub enum Edge {
    Outside = 0,
    Edge1,
    Edge2,
    Edge3,
    Edge4,
    Edge5,
    Edge6,
    Edge7,
    Edge8,
    Edge9,
    Edge10,
    Edge11,
    Edge12,
    Edge13,
    Edge14,
    Edge15,
    Edge16,
    Edge17,
    Edge18,
    Edge19,
    Edge20,
    Edge21,
    Edge22,
}

impl Default for Edge {
    fn default() -> Self {
        Edge::Outside
    }
}

pub const Edges: [Edge; 23] = {
    use Edge::*;
    [
        Outside,
        Edge1,
        Edge2,
        Edge3,
        Edge4,
        Edge5,
        Edge6,
        Edge7,
        Edge8,
        Edge9,
        Edge10,
        Edge11,
        Edge12,
        Edge13,
        Edge14,
        Edge15,
        Edge16,
        Edge17,
        Edge18,
        Edge19,
        Edge20,
        Edge21,
        Edge22,    ]
};

static TILE_DATA: &str = include_str!("../data/e2pieces.txt");

pub fn tiles() -> E2_Tiles {
    let blank: E2_Tile = Default::default();

    let mut tiles = crate::board::Tiles([blank; E2_TILE_COUNT]);
    for (tile_number, line) in TILE_DATA.lines().enumerate() {
        // println!("`{}'", line);
        let mut digits = line
            .trim()
            .splitn(4, " ")
            // .map(|d| { println!("`{}', ", d); d})
            .map(|d| d.parse::<u8>().unwrap())
            .map(|i| Edges[i as usize]);

        let north = digits.next().unwrap();
        let south = digits.next().unwrap();
        let west  = digits.next().unwrap();
        let east  = digits.next().unwrap();

        let tile = crate::board::Tile([north, east, south, west]);

        tiles.0[tile_number] = tile;
    }



    tiles
}