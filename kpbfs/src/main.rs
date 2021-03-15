mod KPBFS;
use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    KPBFS::setup();

    let read_flag = false;

    if read_flag
    {
        let filename = "sample1.in";
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file).lines();
    
        let cols = reader
            .next()
            .map(|v| v.unwrap().parse::<usize>())
            .unwrap()
            .unwrap();
    
        let grid = Grid::from_vec(
            reader.fold(Vec::<char>::new(), |mut acc, x| {
                let v: Vec<char> = x.unwrap().chars().collect();
                acc.extend(&v);
                acc
            }),
            cols,
        );
    
        println!("{:?}", grid);
    }
}
