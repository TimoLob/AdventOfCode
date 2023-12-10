
#[derive(Debug)]
struct Position {
    x : usize,
    y : usize
}
#[derive(Debug,PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
#[derive(Debug,PartialEq, Eq)]
enum Direction {
    N,
    E,
    W,
    S,
    STOP
}
impl Position {
    fn go_direction(&self,dir : &Direction) -> Position{
        //println!("{:?}",self);
        match dir {
            Direction::N => Position {x:self.x , y:self.y-1},
            Direction::E => Position { x: self.x+1, y: self.y },
            Direction::W => Position { x: self.x-1, y: self.y },
            Direction::S => Position { x: self.x, y: self.y+1 },
            Direction::STOP => panic!("Tried to go in STOP direction")
        }
    }

    fn go_direction_and_turn(&self, dir: &Direction, maze : &Vec<Vec<Pipe>>) -> (Position,Direction) {
        use Pipe::*;
        let pos = self.go_direction(dir);
        if *dir == Direction::N {
            return match maze[pos.y][pos.x] {
                Vertical => (pos,Direction::N),
                SW => (pos,Direction::W),
                SE => (pos,Direction::E),
                _ => panic!("Go direction and turn error")
            };
        }
        else if *dir == Direction::S {
            return match maze[pos.y][pos.x] {
                Vertical => (pos,Direction::S),
                NW => (pos,Direction::W),
                NE => (pos,Direction::E),
                _ => panic!("Go direction and turn error")
            };
        }
        else if *dir == Direction::E {
            return match maze[pos.y][pos.x] {
                Horizontal => (pos,Direction::E),
                NW => (pos,Direction::N),
                SW => (pos,Direction::S),
                _ => panic!("Go direction and turn error")
            };
        }
        else if *dir == Direction::W {
            return match maze[pos.y][pos.x] {
                Horizontal => (pos,Direction::W),
                NE => (pos,Direction::N),
                SE => (pos,Direction::S),
                _ => panic!("Go direction and turn error")
            };
        }
        panic!("Go direction and turn got invalid direction");
    }
}



fn check_surrounding(p:&Position, maze: &Vec<Vec<Pipe>>) -> ((Position,Direction),(Position,Direction)) {
    use Pipe::*;
    let mut first_match = (Position {x:0,y:0},Direction::STOP);
    let mut second_match = (Position {x:0,y:0},Direction::STOP);
    {
        // North
        let north = p.go_direction(&Direction::N);
        if matches!(maze[north.y][north.x],Vertical | SE | SW)  {
            let (pos,dir) =  match  maze[north.y][north.x] {
                Vertical => (north,Direction::N),
                SE => (north,Direction::E),
                SW => (north,Direction::W),
                _ => (north,Direction::STOP),
            };
            if first_match.1 == Direction::STOP {
                first_match.0 = pos;
                first_match.1 = dir;
            }
            else {
                second_match.0 = pos;
                second_match.1 = dir;
            }
        }
    }
    {
        // South
        let south = p.go_direction(&Direction::S);
        if matches!(maze[south.y][south.x],Vertical | NE | NW)  {
            let (pos,dir) =  match  maze[south.y][south.x] {
                Vertical => (south,Direction::S),
                NE => (south,Direction::E),
                NW => (south,Direction::W),
                _ => (south,Direction::STOP),
            };
            if first_match.1 == Direction::STOP {
                first_match.0 = pos;
                first_match.1 = dir;
            }
            else {
                second_match.0 = pos;
                second_match.1 = dir;
            }
        }
    }
    {
        // East
        let east = p.go_direction(&Direction::E);
        if matches!(maze[east.y][east.x],Horizontal | NW | SW)  {
            let (pos,dir) =  match  maze[east.y][east.x] {
                Horizontal => (east,Direction::E),
                NW => (east,Direction::N),
                SW => (east,Direction::S),
                _ => (east,Direction::STOP),
            };
            if first_match.1 == Direction::STOP {
                first_match.0 = pos;
                first_match.1 = dir;
            }
            else {
                second_match.0 = pos;
                second_match.1 = dir;
            }
        }
    }
    {
        // West
        let west = p.go_direction(&Direction::W);
        if matches!(maze[west.y][west.x],Horizontal | NE | SE)  {
            let (pos,dir) =  match  maze[west.y][west.x] {
                Horizontal => (west,Direction::W),
                NE => (west,Direction::N),
                SE => (west,Direction::S),
                _ => (west,Direction::STOP),
            };
            if first_match.1 == Direction::STOP {
                first_match.0 = pos;
                first_match.1 = dir;
            }
            else {
                second_match.0 = pos;
                second_match.1 = dir;
            }
        }
    }

    return (first_match,second_match);
}


fn solve(input : &str) -> usize {
    let pipe_maze_chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();

    let pipe_maze : Vec<Vec<Pipe>>= pipe_maze_chars.iter().map(|vec| {
        vec.iter().map(|c|{
            match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NE,
                'J' => Pipe::NW,
                '7' => Pipe::SW,
                'F' => Pipe::SE,
                '.' => Pipe::Ground,
                'S' => Pipe::Start,
                value => panic!("Found char{}", value)
            }
        }).collect::<Vec<Pipe>>() 
    }).collect();
    println!("{:?}",pipe_maze);

    let mut start: Position = Position { x: 0, y: 0 };
    let mut pos1 = Position { x: 0, y: 0 };
    let mut dir1 = Direction::STOP; 
    let mut pos2 = Position { x: 0, y: 0 };
    let mut dir2 = Direction::STOP;
    
    for (y,line) in pipe_maze.iter().enumerate() {
        for (x,pipe) in line.iter().enumerate() {
            print!("{:?} ",pipe);
            if *pipe == Pipe::Start {
                start.x = x;
                start.y = y;
                ((pos1,dir1),(pos2,dir2)) = check_surrounding(&start, &pipe_maze);
            }
            
        }
        println!();
    }
    println!("Start : {:?}",start);
    println!("Pos1 : {:?} {:?}",pos1,dir1);
    println!("Pos2 : {:?} {:?}",pos2,dir2);
    let mut step = 1;
    while !(pos1.x == pos2.x && pos1.y==pos2.y) {
        (pos1,dir1) = pos1.go_direction_and_turn(&dir1, &pipe_maze);
        (pos2,dir2) = pos2.go_direction_and_turn(&dir2, &pipe_maze);
        step+=1;
    }
    return step;
}


fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}",result);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let example = include_str!("../../example1.txt");
        assert_eq!(solve(example), 4);
    }
    #[test]
    fn example1_easy() {
        let example = include_str!("../../example1_easy.txt");
        assert_eq!(solve(example), 4);
    }

}