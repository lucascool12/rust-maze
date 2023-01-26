mod maze;
mod grid_visualizer;

use std::env;

fn main() -> Result<(), std::io::Error>{
    let args: Vec<String> = env::args().collect();
    
    let size: u16 = match args.get(1){
        Some(s) => match s.parse::<u16>(){
            Ok(s) => s,
            Err(_) => {
                println!("Not a correct size");
                return Ok(());
            }
        },
        None => 4
    };
    
    return maze::maze(size);
}
