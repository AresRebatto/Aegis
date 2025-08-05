use ratatui::{
    Frame,
    prelude::*,
    widgets::{block::{Position, Title},Block, Paragraph, Padding},
    style::{Style, Color},
};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::time::Duration;


use aegis_core::engine::structs::user::*;
use super::super::models::login_result::LoginResult;



pub fn login_page(frame: &mut Frame, selected_user: i32)-> LoginResult{

    //That'll be a User vec
    let users: Vec<User> = vec![];
    
    let title_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(frame.area())[0];

    let title = Paragraph::new("Log-in")
    .block(Block::default().padding(Padding::horizontal(2)))
    .alignment(Alignment::Center);

    let box_: Block<'_> = Block::new()
                            .title_bottom("Ctrl+Q - Exit\t\t\t")
                            .title_bottom("Up/Down - Change User");
    
    let user_area_width = frame.area().width/3 as u16;
    let new_user_area = Rect::new(0, 3, user_area_width, 1);
    let new_user_text = Paragraph::new("Create new user");
    
    if selected_user == 0{
        frame.render_widget(new_user_text.black().on_white(), new_user_area);
    }else{
        frame.render_widget(new_user_text.white().on_black(), new_user_area);
    }
    
    for user in &users{
        let user_area = Rect::new(0, 3+user.id as u16, user_area_width, 1);
        let user_text = Paragraph::new(user.name.clone());
        if selected_user == user.id{
            frame.render_widget(user_text.black().on_white(), user_area);
        }else{
           frame.render_widget(user_text.white().on_black(), user_area); 
        }
    }
    
    frame.render_widget(box_, frame.area());
    frame.render_widget(title, title_area);
    

    

    LoginResult::Ok

}