use std::str::FromStr;
#[derive(Debug)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEastL,
    NorthWestJ,
    SouthWest7,
    SouthEastF,
    Ground,
    Animal,
}

impl FromStr for Pipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Pipe::Vertical),
            "-" => Ok(Pipe::Horizontal),
            "L" => Ok(Pipe::NorthEastL),
            "J" => Ok(Pipe::NorthWestJ),
            "7" => Ok(Pipe::SouthWest7),
            "F" => Ok(Pipe::SouthEastF),
            "." => Ok(Pipe::Ground),
            "S" => Ok(Pipe::Animal),
            _ => Err(format!("Character {s} isn't a known pipe type")),
        }
    }
}
