use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{text::Text, Frame};
use std::time::Duration;
use std::fs::File;
use std::fs::create_dir;
pub mod pages;
pub mod models;
use pages::login_page::login_page;

fn main() {
    
    let mut terminal = ratatui::init();
    let mut selected_user = 0;
    loop {
        if let Err(e) = terminal.draw(|frame| {
            login_page(frame, selected_user);
        }){
            eprintln!("An error occurred while attempting to start the application: {}", e)
        }
        
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                
                match key_event.code{
                    KeyCode::Up => {
                        if selected_user > 0{
                            selected_user -=1
                        }
                    },
                    KeyCode::Down => {
                        //len of get_users
                        if selected_user < 3 as i32{
                            selected_user +=1
                        }
                    }
                    _ => {}
                }
                if key_event.code == KeyCode::Char('q') && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }       
        }
        
    }
    ratatui::restore();
}
