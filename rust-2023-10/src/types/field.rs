use crate::types::{coord::Coord, pipe::Pipe};

// x y plane that starts in the top left corner. [0,0] is the very top left square
pub struct Field(pub Vec<Vec<Pipe>>);

impl Field {
    fn at(&self, loc: &Coord) -> &Pipe {
        &self.0[loc.x][loc.y]
    }

    fn next(&self, prev_loc: Coord, curr_loc: Coord) -> Coord {
        let curr_pipe = self.at(&curr_loc);

        match curr_pipe {
            Pipe::Vertical => match prev_loc == curr_loc.south() {
                true => curr_loc.north(),
                false => curr_loc.south(),
            },
            Pipe::Horizontal => match prev_loc == curr_loc.west() {
                true => curr_loc.east(),
                false => curr_loc.west(),
            },
            Pipe::NorthEastL => match prev_loc == curr_loc.north() {
                true => curr_loc.east(),
                false => curr_loc.north(),
            },
            Pipe::NorthWestJ => match prev_loc == curr_loc.north() {
                true => curr_loc.west(),
                false => curr_loc.north(),
            },
            Pipe::SouthWest7 => match prev_loc == curr_loc.south() {
                true => curr_loc.west(),
                false => curr_loc.south(),
            },
            Pipe::SouthEastF => match prev_loc == curr_loc.south() {
                true => curr_loc.east(),
                false => curr_loc.south(),
            },
            Pipe::Ground => {
                panic!("Current Location is the ground")
            }
            Pipe::Animal => {
                panic!("Current location is the animal")
            }
        }
    }
}
