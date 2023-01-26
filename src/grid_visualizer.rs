pub mod grid_view{

    use tui::{
        style::{Style},
        buffer::{Buffer},
        layout::{Rect},
        text::{Span},
        widgets::{Widget},
    };

    pub trait Block{
        fn get_style(&self) -> (Style, &str);
    }

    pub trait GridModel{
        fn get_piece(&self, x: u16, y:u16) -> &dyn Block;
        fn get_size(&self) -> u16;
        fn get_tile_size(&self) -> u16;
    }


    pub struct GridVisualizer<M:GridModel>{
        pub maze: M,
        pub message : Option<String>
    }

    impl<M:GridModel> GridVisualizer<M>{
        fn get_style(&self, x: u16, y: u16) -> (Style, &str){
            self.maze.get_piece(x,y).get_style()
        }
    }

    pub struct MazeWidget<'a, M:GridModel>{
        pub visual: &'a GridVisualizer<M>
    }

    impl<'a, M:GridModel> Widget for MazeWidget<'a, M>{
        fn render(self, area: Rect, buf: &mut Buffer){
            let size_tile = self.visual.maze.get_tile_size();
            let mut maze_size = if area.width/2 < area.height-1{
                    area.width/2
                }else{
                    area.height-1
                };
            
            let mut tile_size = maze_size/(self.visual.maze.get_size()) as u16;

            let block_size = tile_size/(size_tile) as u16;

            if block_size == 0 {
                buf.set_span(0, 0, &Span::raw("Terminal too small!"), area.width);
                return;
            }

            tile_size = block_size*size_tile as u16;

            maze_size = tile_size*self.visual.maze.get_size() as u16;
            for i in 0..maze_size{
                for j in 0..maze_size*2{
                    let styl = self.visual.get_style(j/(2*block_size), i/(block_size));
                    buf.get_mut(j, i).set_style(styl.0)
                                    .set_symbol(styl.1);
                }
            }

            match &self.visual.message{
                Some(mes) => {buf.set_span(0, maze_size, &Span::raw(mes), area.width);},
                None => ()
            };
        }
    }

    pub enum ReadResizeEvent{
        Char(char),
        Resize(u16,u16)
    }
    use crossterm::event::{read, Event, KeyCode};
    pub fn read_key_and_watch_for_resize() -> crossterm::Result<ReadResizeEvent>{
        loop {
            // `read()` blocks until an `Event` is available
            match read()? {
                Event::Key(event) => {
                    match event.code{
                        KeyCode::Char(c) => break Ok(ReadResizeEvent::Char(c)),
                        _ => ()
                    }
                },
                Event::Resize(x,y) => break Ok(ReadResizeEvent::Resize(x,y)),
                _ => ()
            }
        }
    }
    use tui::Terminal;
    use tui::backend::Backend;
    pub fn draw_and_read_input<M:GridModel, B:Backend>(terminal:&mut Terminal<B>, vis:&mut GridVisualizer<M>) -> Result<char, std::io::Error>{
        match terminal.backend_mut().set_cursor(0,0){_ => ()};
        match terminal.backend_mut().hide_cursor(){_ => ()};

        loop{
            terminal.draw(|f| {
                let size = f.size();
                let maze_widget = MazeWidget{
                    visual : &vis
                };
                f.render_widget(maze_widget, size);
            })?;
            vis.message = None;
            match read_key_and_watch_for_resize(){
                Ok(res) => match res{
                    ReadResizeEvent::Char(c) => break Ok(c),
                    ReadResizeEvent::Resize(_,_) => continue
                },
                Err(er) => {
                    break Err(er)
                }
            };
        }
    }
}