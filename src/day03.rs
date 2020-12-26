use std::io::{prelude::* , BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    fn expected_maptile() -> MapTile {
        MapTile {
            content:  vec![
                MapRow {
                    content: vec!['.', '#', '.', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.'] },
                MapRow {
                    content: vec!['#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '.', '.', '.', '#', '#', '.', '#', '#', '.'] },
                MapRow {
                    content: vec!['#', '.', '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '#', '.', '.', '.', '.', '.', '#', '#'] },
                MapRow {
                    content: vec!['#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '#', '#', '#'] }
            ],
        }
    }

    #[test]
    fn ingest_maptile() {
        assert_eq!(expected_maptile(), MapTile::new(crate::utils::get_filepath("src/tests/test_map_data.txt")))
    }

    #[test]
    fn map_new() {
        let expected = Map {
            content: vec![expected_maptile()],
        };

        assert_eq!(expected, Map::new(crate::utils::get_filepath("src/tests/test_map_data.txt")));
    }

    #[test]
    fn map_extend() {
        let expected = Map {
            content: vec![expected_maptile(), expected_maptile()]
        };

        let mut initial_map = Map {
            content: vec![expected_maptile()]
        };

        assert_eq!(expected, initial_map.extend());
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Map  {
    content: Vec<MapTile>,
}

#[derive(Debug, PartialEq, Clone)]
struct MapTile{
    content: Vec<MapRow>,
}

#[derive(Debug, PartialEq, Clone)]
struct MapRow {
    content: Vec<char>,
}

impl Map {
    fn new(filepath: std::path::PathBuf) -> Self {
        Self {
            content: vec![MapTile::new(filepath)],
        }
    }

    fn extend(&self) -> Self{
        let mut newmap: Map = self.clone();
        let mut init_maptile: MapTile = self.content[0].clone();

        newmap.content.push(init_maptile);

        newmap
    }
}

impl MapTile {
    fn new(filepath: std::path::PathBuf) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let lines = BufReader::new(file).lines();

        let mut thistile: Vec<MapRow> = vec![];

        for (_, line) in lines.enumerate() {
            let line_ = line.unwrap();
            let thisline = line_.chars();
            let mut thisrow: Vec<char> = vec![];

            for (_, c) in thisline.enumerate() {
                thisrow.push(c);
            }

            thistile.push(MapRow { content: thisrow });
        }
        
        return Self {
            content: thistile,
        }
    }
}

pub fn day3() {
    let thistile = MapTile::new(crate::utils::get_filepath("src/tests/test_map_data.txt"));
    println!("{:?}", thistile);
}
