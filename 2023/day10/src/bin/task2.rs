use std::vec;
use std::collections::VecDeque;
#[derive(Debug)]
struct Position {
    x : usize,
    y : usize
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum MazeArea {
    Inside,
    Outside,
    Loop,
    Counts,
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



fn check_surrounding(p:&Position, maze: &Vec<Vec<Pipe>>) -> (Pipe,((Position,Direction),(Position,Direction))) {
    use Pipe::*;
    let mut first_match = (Position {x:0,y:0},Direction::STOP);
    let mut second_match = (Position {x:0,y:0},Direction::STOP);
    let mut n = false;
    let mut s = false;
    let mut e = false;
    let mut w = false;
    if p.y >0
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
            n = true;
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
            s = true;
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
            e = true;
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
            w = true;
        }
    }
    let mut start_pipe = Pipe::Start;
    if n {
        if w {
            start_pipe = NW;
        } else if e{
            start_pipe = NE;
        }
        else if s {
            start_pipe = Vertical;
        }
    }
    else if s {
        if w {
            start_pipe = SW;
        }
        else if e {
            start_pipe = SE;
        }
    }
    else {
        start_pipe = Horizontal;
    }
    return (start_pipe,(first_match,second_match));
}

fn fill(area : &mut Vec<Vec<MazeArea>>,x:usize,y:usize){
    use MazeArea::*;
    //println!("{x} {y}");
    let mut positions : VecDeque<Position> = VecDeque::from([Position {x,y}]);

    while !positions.is_empty() {
        let pos = positions.pop_front().expect("Queue");
        let x = pos.x;
        let y = pos.y;
        if area[pos.y][pos.x] == Loop || area[pos.y][pos.x] == Outside {
            continue;
        } 
        else {
            area[y][x] = Outside;
            if x<area[pos.y].len()-1 {
                positions.push_back(Position { x: x+1, y: y });

            }
            if x > 0 {
                positions.push_back(Position { x: x-1, y: y });

            }
            if y<area.len()-1 {
                positions.push_back(Position { x: x, y: y+1 });
            }
            if y > 0 {
                positions.push_back(Position { x: x, y: y-1 });
            }
            
        }
    }
    
    return;
}


fn enter_pipe(area: &mut Vec<Vec<MazeArea>>,tile : &Pipe, x:usize, y:usize) {
    use MazeArea::*;
    //println!("{:?} {x} {y}",tile);
    let big_x = 3*x+1;
    let big_y = 3*y+1;
    //println!("{big_x} {big_y}");
    match tile {
        Pipe::Horizontal => {
            area[big_y][big_x-1]=Loop;
            area[big_y][big_x]=Loop;
            area[big_y][big_x+1]=Loop;
            },
        Pipe::Vertical => {
            area[big_y-1][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y+1][big_x]=Loop;
        }
        Pipe::NE => {
            area[big_y-1][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y][big_x+1]=Loop;
        },
        Pipe::NW => {
            area[big_y-1][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y][big_x-1]=Loop;
        },
        Pipe::SE => {
            area[big_y+1][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y][big_x+1]=Loop;
        },
        Pipe::SW => {
            area[big_y+1][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y][big_x-1]=Loop;
        },
        Pipe::Start => {
            area[big_y][big_x]=Loop;
            area[big_y][big_x]=Loop; 
            area[big_y][big_x]=Loop;
        },
        _ => panic!("Invalid pipe {:?}",tile)

    }
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
    //println!("{:?}",pipe_maze);
    

    let mut area:Vec<Vec<MazeArea>> = vec![];
    for line in pipe_maze.iter() {
        for _ in 0..4 {
        let mut v: Vec<MazeArea> = vec![];
        for _ in 0..line.len()*3 {
            v.push(MazeArea::Inside);
        }
        area.push(v);
        }
    }

    let mut start: Position = Position { x: 0, y: 0 };
    let mut pos1 = Position { x: 0, y: 0 };
    let mut dir1 = Direction::STOP; 
    let mut pos2 = Position { x: 0, y: 0 };
    let mut dir2 = Direction::STOP;
    let mut start_pipe = Pipe::Start;
    for (y,line) in pipe_maze.iter().enumerate() {
        for (x,pipe) in line.iter().enumerate() {
            //print!("{:?} ",pipe);
            if *pipe == Pipe::Start {
                start.x = x;
                start.y = y;
                (start_pipe,((pos1,dir1),(pos2,dir2))) = check_surrounding(&start, &pipe_maze);
                enter_pipe(&mut area, &start_pipe, start.x, start.y);
            }
            
        }
        //println!();
    }
    println!("Start : {:?}",start);
    println!("Pos1 : {:?} {:?}",pos1,dir1);
    println!("Pos2 : {:?} {:?}",pos2,dir2);

    //area[pos1.y][pos1.x] = MazeArea::Loop;
    //area[pos2.y][pos2.x] = MazeArea::Loop;
    enter_pipe(&mut area, &pipe_maze[pos1.y][pos1.x], pos1.x, pos1.y);
    enter_pipe(&mut area, &pipe_maze[pos2.y][pos2.x], pos2.x, pos2.y);
    let mut step = 1;
    while !(pos1.x == pos2.x && pos1.y==pos2.y) {
        (pos1,dir1) = pos1.go_direction_and_turn(&dir1, &pipe_maze);
        (pos2,dir2) = pos2.go_direction_and_turn(&dir2, &pipe_maze);
        //println!("Pos1 : {:?} {:?}",pos1,dir1);
        //println!("Pos2 : {:?} {:?}",pos2,dir2);
        enter_pipe(&mut area, &pipe_maze[pos1.y][pos1.x], pos1.x, pos1.y);
        enter_pipe(&mut area, &pipe_maze[pos2.y][pos2.x], pos2.x, pos2.y);
        step+=1;
    }
    //println!("{:?}",area);

    let mut tiles = 0;
    fill(&mut area,0,0);

    for y in 1..area.len()-1 {

        for x in 1..area[y].len()-1 {
            let tile = area[y][x];
            if tile == MazeArea::Inside {
                if area[y][x-1] == MazeArea::Inside && area[y][x+1] == MazeArea::Inside && area[y-1][x] == MazeArea::Inside && area[y+1][x] == MazeArea::Inside &&
                   area[y+1][x+1] == MazeArea::Inside && area[y-1][x+1] == MazeArea::Inside && area[y+1][x-1] == MazeArea::Inside && area[y-1][x-1] == MazeArea::Inside {
                    tiles+=1;
                    // area[y][x]=MazeArea::Counts
                   }
            }
            
            print!("{} ",match area[y][x] {
                MazeArea::Inside => 'I',
                MazeArea::Loop => '#',
                MazeArea::Outside => '.',
                MazeArea::Counts => 'O'
            });
            
        }
        println!();
    }



    //println!("{:?}",area);
    return tiles/9;
}


fn main() {
    //println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let result = solve(input);
    println!("Result {}",result);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example2() {
        let example = include_str!("../../example2.txt");
        assert_eq!(solve(example), 10);
    }


}