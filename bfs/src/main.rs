
use grid::Grid;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

mod temp_structs;
use temp_structs::{Node, Point};

fn is_valid(x: usize, y: usize, graph: &Grid<char>) -> bool {
    // Don't need to check below 0 since unsigned
    let in_bounds = x < graph.size().0 && y < graph.size().0;
    if !in_bounds {
        return false;
    }

    return in_bounds && graph[x][y] != 'W';
}

fn main() {
    let filename = "../data/medium1.in";
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file).lines();

    let cols = reader
        .next()
        .map(|v| v.unwrap().parse::<usize>())
        .unwrap()
        .unwrap();

    let graph = Grid::from_vec(
        reader.fold(Vec::<char>::new(), |mut acc, x| {
            let v: Vec<char> = x.unwrap().chars().collect();
            acc.extend(&v);
            acc
        }),
        cols,
    );

    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut closed: HashMap<Point, Node> = HashMap::new();

    let mut startx: i32 = 0;
    let mut starty: i32 = 0;
    let mut endx: i32 = 0;
    let mut endy: i32 = 0;

    for i in 0..graph.size().0 {
        for j in 0..graph.size().0 {
            if graph[i][j] == 'S' {
                startx = i as i32;
                starty = j as i32;
            }
            if graph[i][j] == 'E' {
                endx = i as i32;
                endy = j as i32;
            }
        }
    }

    println!("{} {} {} {}", startx, starty, endx, endy);

    let mut start = Node::new(startx, starty, 0, 0, 0, Point::default());
    let end = Node::new(endx, endy, 0, 0, 0, Point::default());

    queue.push_back(start);

    while queue.len() > 0
    {
        // Know queue.len() > 0 so can force unwrap.
        let pop = queue.pop_front().unwrap();
        if pop.position.x == end.position.x && pop.position.y == end.position.y
        {
            println!("Goal found {} steps!", pop.g);
            break;
        }

        if closed.contains_key(&pop.position)
        {
            if closed.get(&pop.position).unwrap().g <= pop.g
            {
                continue;
            }
        }

        closed.insert(pop.position, pop);

        let adjacent = vec![(0, 1), (-1, 0), (1, 0), (0, -1)];

        for (x,y) in adjacent
        {
            let n_x = pop.position.x + x;
            let n_y = pop.position.y + y;

            if n_x < 0 || n_y < 0
            {
                continue;
            }

            if is_valid(n_x as usize, n_y as usize, &graph)
            {
                let mut n_prime = Node::new(n_x, n_y, 0, pop.g + 1, 0, pop.position);

                if closed.contains_key(&n_prime.position)
                {
                    if closed.get(&n_prime.position).unwrap().g < n_prime.g
                    {
                        continue;
                    }
                }
                closed.insert(n_prime.position, n_prime);
                queue.push_back(n_prime);
            }
        }
    }

}
