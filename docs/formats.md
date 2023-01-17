# File Formats

To facilitate data exchange, the [e2rs] library re-uses and/or defines a number of standard file formats.
These are designed to be both human-readable and machnie-readable, and to be editable in a text editor.

## Tile Indexing

Tiles are indexed from 1, not 0.
Where not otherwise specified, tile indexes refer to tiles in the tileset used in the Eternity 2 Puzzle.

## Edge Indexing

Edges are indexed from 0, with 0 always referring to the "outside" edge type.
When the edge set is not explicitly given, it defaults to the edges used in the Eternity 2 Puzzle.

## Board Cell Indexing

Cells within boards are numbered as *column*,*row* or equivalently *x*,*y*, starting at the top-left corner.
The coordinates index from 1.
So cell 1,1 is the top-left cell of the puzzle board.

## Rotations

Rotations are indexed from 0, meaning no rotation to 3, meaning 270°.
By default, rotations are anti-clockwise.
Individual tools may provide switches to over-ride this.

## Sides

Tiles have 4 sides, each of which is marked with an edge.
These are numbered 0..3, going clockwise.
North, East, South, West.

## Comment lines

All formats accept comment lines.
These begin with `#`.
Comment lines are skipped from further parsing, and have no semantic import.
Blank lines are not comment lines, and will be treated by parsers as parsed.

## Board shape

Board shapes are given by one of two dimensions lines:

    - <size>
    - <columns> <rows>
  
If the single-value form is given, the board is taken to be square.
If the two-value form is given, the board is taken to be rectangular.
When dimension lines are optional, then the shape defaults to the Eternity 2 Puzzle dimensions of 16x16.

## Clue

A clue is the placement of a tile at a particular position and orientation.
Each clue line is a single clue in the following format:

    <tile_number: u8> <column: u8> <row: u8> <rotation: {0,1,2,3}>

A clue can be validly placed on a board if the tile nubmer refers to a tile in the tileset for
that board, and if the column and row are within range for the board's dimensions.
Rotations are taken to be in the anti-clockwise direction by default, or unless explicitly marked as clockwise by import options.

A valid clue file is a series of lines, each one containing exactly one clue.

## Tile

A tile is described by listing its 4 edges.
Each tile line is a single tile in the following format:

    <north: u8> <east: u8> <south: u8> <west: u8>

A tile is valid with respect to an edge set if each of the edges of a tile is a valid index into
the set of edges.

A valid tiles file is a list of valid tile lines, each one containing exactly one tile.

## BoardSpec

A board specification file is an optional board shape line followed by any number of tile lines.

## Solution

A solution file is an optional board shape line followed by any number of tile lines.
