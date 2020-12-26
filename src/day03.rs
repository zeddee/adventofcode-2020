use std::io::{prelude::* , BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_maptile() {
        let expected = MapTile {
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
        };

        assert_eq!(expected, MapTile::new(crate::utils::get_filepath("src/tests/test_map_data.txt")))
    }

}

struct Map  {
    content: Vec<MapTile>,
}

#[derive(Debug, PartialEq)]
struct MapTile{
    content: Vec<MapRow>,
}

#[derive(Debug, PartialEq)]
struct MapRow {
    content: Vec<char>,
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
