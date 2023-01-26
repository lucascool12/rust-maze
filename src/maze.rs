
use crate::grid_visualizer::grid_view;

mod inner_maze{
    
    use rand::Rng;
    use std::fmt;
    use tui::{
        style::{Style,Color},
        symbols::block
    };
    
    const SIZETILE: usize = 3;

    #[derive(Copy, Clone)]
    pub enum Types{
        Empty,
        Wall,
        Player,
        Gold
    }

    impl fmt::Display for Types{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            match self{
                Types::Empty => write!(f, "   "),
                Types::Wall => write!(f, " # "),
                Types::Player => write!(f," p "),
                Types::Gold => write!(f," g ")
            }
        }
    }
    
    impl super::grid_view::Block for Types{
        fn get_style(&self) -> (Style, &str){
            match self{
                Types::Empty => (Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Black),block::FULL),
                Types::Wall => (Style::default()
                                    .fg(Color::White)
                                    .bg(Color::Black),block::FULL),
                Types::Player => (Style::default()
                                    .fg(Color::Green)
                                    .bg(Color::Black),block::FULL),
                Types::Gold => (Style::default()
                                    .fg(Color::Yellow)
                                    .bg(Color::Black),block::FULL)
            }
        }
    }

    #[derive(Clone)]
    pub struct Tile{
        place : [[Types; SIZETILE]; SIZETILE]
    }

    impl Tile{
        fn rotate(mut self, rotate:u8) -> Tile{
            for _ in 0..rotate{
                let copy = self.clone();
                for i in 0..SIZETILE{
                    for j in 0..SIZETILE{
                        self.place[j][SIZETILE - i - 1] = copy.place[i][j];
                    }
                }
            }
            self
        }
    }
    
    
    impl fmt::Display for Tile{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            for i in 0..SIZETILE{
                for j in 0..SIZETILE{
                    write!(f,"{}", self.place[j][i])?;
                }
                write!(f,"\n")?;
            }
            std::result::Result::Ok(())
        }
    }

    
    pub struct Maze{
        tiles : Vec<Tile>,
        size : usize,
        player : (u16, u16),
        gold : Option<(u16, u16)>,
    }

    impl super::grid_view::GridModel for Maze{
        fn get_size(&self) -> u16{
            self.size as u16
        }

        fn get_tile_size(&self) -> u16{
            SIZETILE as u16
        }

        fn get_piece(&self, x: u16, y: u16) -> &dyn super::grid_view::Block{
            &self.tiles[(x/(SIZETILE as u16)) as usize + (y/(SIZETILE as u16)) as usize * self.size].place[(x%SIZETILE as u16) as usize][(y%SIZETILE as u16) as usize]
        }
    }

    impl Maze{
        fn get_tile(&mut self, x:u16, y:u16) -> Option<&mut Tile>{
            self.tiles.get_mut(x as usize + y as usize*self.size)
        }

        fn get_player_tile(&mut self) -> &mut Tile{
            self.get_tile(self.player.0/SIZETILE as u16, self.player.1/SIZETILE as u16).unwrap()
        }

        pub fn move_player(&mut self, dir_in:u8) -> Result<Types,&str>{
            let dir:(i16,i16) = 
                match dir_in{
                    0 => (1,0), //right
                    1 => (0,-1), //up
                    2 => (-1,0), //left
                    3 => (0,1), //down
                    _ => (0,0)
                };

            let mut new_player_pos = (self.player.0 as i16 + dir.0, self.player.1 as i16 + dir.1);
            

            if new_player_pos.0 >= self.size as i16*SIZETILE as i16{
                new_player_pos = (new_player_pos.0 - self.size as i16*SIZETILE as i16, new_player_pos.1);
            }else if new_player_pos.1 >= self.size as i16*SIZETILE as i16{
                new_player_pos = (new_player_pos.0, new_player_pos.1 - self.size as i16*SIZETILE as i16);
            }else if new_player_pos.0 < 0{
                new_player_pos = (new_player_pos.0 + self.size as i16*SIZETILE as i16, new_player_pos.1);
            }else if new_player_pos.1 < 0{
                new_player_pos = (new_player_pos.0, new_player_pos.1 + self.size as i16*SIZETILE as i16);
            }
            let new_player_tile_pos = (new_player_pos.0%SIZETILE as i16, new_player_pos.1%SIZETILE as i16);
            // dbg!(new_player_pos);
            let new_player_tile = 
                match self.get_tile(new_player_pos.0 as u16/SIZETILE as u16, new_player_pos.1 as u16/SIZETILE as u16){
                    Some(tile) => tile,
                    None => {
                        return Err("Out of bounds movement!");
                    }
                };

            match new_player_tile.place[new_player_tile_pos.0 as usize][new_player_tile_pos.1 as usize]{
                Types::Empty => {
                    new_player_tile.place[new_player_tile_pos.0 as usize][new_player_tile_pos.1 as usize] = Types::Player;
                    let x = self.player.0%SIZETILE as u16;
                    let y = self.player.1%SIZETILE as u16;
                    self.get_player_tile().place[x as usize][y as usize] = Types::Empty;
                    self.player = (new_player_pos.0 as u16, new_player_pos.1 as u16);
                    Ok(Types::Empty)
                }
                Types::Gold => {
                    new_player_tile.place[new_player_tile_pos.0 as usize][new_player_tile_pos.1 as usize] = Types::Player;
                    let x = self.player.0%SIZETILE as u16;
                    let y = self.player.1%SIZETILE as u16;
                    self.get_player_tile().place[x as usize][y as usize] = Types::Empty;
                    self.player = (new_player_pos.0 as u16, new_player_pos.1 as u16);
                    self.gold = None;
                    Ok(Types::Gold)
                }
                Types::Player => Ok(Types::Player),
                Types::Wall => Ok(Types::Wall)
            }
        }
    }

    impl fmt::Display for Maze{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            let display_height = 
                if self.size != 0 {
                    self.tiles[0].to_string().lines().count()
                }else{
                    0
                };
                
            for i in 0..self.size{
                for u in 0..display_height{
                    for j in 0..self.size{
                        write!(f,"{}", self.tiles[j + self.size*i].to_string().lines().nth(u).unwrap())?
                    }
                    write!(f,"\n")?
                }
            }
            std::result::Result::Ok(())
        }
    }

    pub fn create_templates() -> Vec<Tile>{
        let mut temps = Vec::new();
        let lshape = Tile{
            place : [[Types::Wall, Types::Empty, Types::Wall],
                     [Types::Wall, Types::Empty, Types::Empty],
                     [Types::Wall, Types::Wall, Types::Wall]]
        };
        let cross = Tile{
            place : [[Types::Wall, Types::Empty, Types::Wall],
                     [Types::Empty, Types::Empty, Types::Empty],
                     [Types::Wall, Types::Empty, Types::Wall]]
        };
        let straight = Tile{
            place : [[Types::Wall, Types::Wall, Types::Wall],
                     [Types::Empty, Types::Empty, Types::Empty],
                     [Types::Wall, Types::Wall, Types::Wall]]
        };
        let empty = Tile{
            place : [[Types::Empty, Types::Empty, Types::Empty],
                     [Types::Empty, Types::Empty, Types::Empty],
                     [Types::Empty, Types::Empty, Types::Empty]]
        };
        temps.push(lshape);
        temps.push(cross);
        temps.push(straight);
        temps.push(empty);
        temps
    }

    pub fn create_maze(size : u16) -> Maze{
        let temps = create_templates();
        let mut rng = rand::thread_rng();
        let mut maze = Maze{
            tiles : Vec::new(),
            size : size as usize,
            player : (0,0),
            gold : None,
        };
        for _ in 0..maze.size{
            for _ in 0..maze.size{
                maze.tiles.push((&temps[rng.gen_range(0, temps.len())]).clone().rotate(rng.gen_range(0,4)));
            }
        }

        let player = (rng.gen_range(0, size) as u16, rng.gen_range(0, size)as u16);
        let mut gold = (rng.gen_range(0, size) as u16, rng.gen_range(0, size) as u16);
        while player.0 == gold.0 && player.1 == gold.1{
            gold = (rng.gen_range(0, size) as u16, rng.gen_range(0, size)as u16);
        }
        maze.get_tile(player.0, player.1).unwrap().place[SIZETILE/2][SIZETILE/2] = Types::Player;
        maze.get_tile(gold.0, gold.1).unwrap().place[SIZETILE/2][SIZETILE/2] = Types::Gold;
        maze.player = (player.0*SIZETILE as u16 + 1,player.1*SIZETILE as u16 + 1);
        maze.gold = Some((gold.0*SIZETILE as u16 + 1,gold.1*SIZETILE as u16 + 1));
        maze
    }
}

use crate::maze::inner_maze as maze;
use std::io;
use tui::{
    backend::{CrosstermBackend},
    Terminal
};


pub fn maze(size : u16) -> Result<(), io::Error>{
    let maze = maze::create_maze(size);
    let mut vis = grid_view::GridVisualizer{
        maze : maze,
        message : None
    };
    print!("{esc}c", esc = 27 as char);
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    loop{
        
        let c = grid_view::draw_and_read_input(&mut terminal, &mut vis);

        let dir = match c{
            Ok(ch) => match ch{  
                'd' => 0,
                'w' => 1,
                'a' => 2,
                's' => 3,
                'n' => {
                    vis.maze = maze::create_maze(size);
                    continue;
                },
                'q' => break,
                _ => {
                    vis.message = Some(String::from("Wrong input"));
                    continue;
                }
            }
            Err(er) => {
                println!("Something went wrong: {}", er);
                break;
            }
        };

        match vis.maze.move_player(dir){
            Ok(types) => match types{
                maze::Types::Wall => vis.message = Some(String::from("Cannot move through walls")),
                maze::Types::Player => vis.message = Some(String::from("Moving into same position should not be possible")),
                maze::Types::Gold => vis.message = Some(String::from("You have found the gold!")),
                maze::Types::Empty => (),
            },
            Err(message) => vis.message = Some(message.to_string())
        }
    }
    print!("{esc}c", esc = 27 as char);
    Ok(())
}




