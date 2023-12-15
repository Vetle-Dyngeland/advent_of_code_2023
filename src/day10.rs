const INPUT: &str = "input_files/day10.txt";

type Maze = Vec<Vec<char>>;
type Connection = [Direction; 2];

pub fn run() {
    println!("--- DAY 10 ---");
    super::solve_and_time("Part 1: ", part1);
    super::solve_and_time("Part 2: ", part2);
}

fn part1() -> String {
    let maze = get_maze();
    let start_index = starting_index(&maze);
    let traversal = traverse_maze(&maze, start_index);
    println!("{start_index:?}");

    "".to_string()
}

fn get_maze() -> Maze {
    std::fs::read_to_string(INPUT)
        .unwrap()
        .split('\n')
        .map(|s| s.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn starting_index(maze: &Maze) -> [usize; 2] {
    for x in 0..maze.len() {
        for y in 0..maze[x].len() {
            if maze[x][y] == 'S' {
                return [x, y]
            }
        }
    }
    panic!("Could not find a starting index");
}

fn traverse_maze(maze: &Maze, start: [usize; 2]) -> usize {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {

        }
    }
}

fn part2() -> String {
    "".to_string()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn from_isize_pair([x, y]: [isize; 2]) -> Option<Self> {
        use Direction::*;
        Some(match (x, y) {
            (0, -1) => North,
            (0, 1) => South,
            (-1, 0) => West,
            (1, 0) => East,
            _ => return None
        })
    }
}

fn char_to_connection(c: char) -> Option<Connection> {
    use Direction::*;
    Some(match c {
        '|' => [North, South],
        '-' => [East, West],
        'L' => [North, East],
        'J' => [North, West],
        '7' => [South, West],
        'F' => [South, East],
        _ => return None
    })
}
