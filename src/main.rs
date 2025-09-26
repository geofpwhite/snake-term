mod snake;
use core::time;
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::enable_raw_mode,
    },
    layout::{Position, Rect},
    prelude::CrosstermBackend,
    style::{Color, Style, Styled},
    widgets::Block,
};
use snake::Snake;
use std::{
    io::{self, Stdout},
    sync::{Arc, Mutex},
    thread::{self, sleep},
};
fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let stdout: Stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let h = terminal.size().unwrap().height;
    let w = terminal.size().unwrap().width;
    let s = Arc::new(Mutex::new(Snake::new(w as usize, h as usize)));
    let s1 = Arc::clone(&s);
    terminal.clear().unwrap_or(());
    let t1 = thread::spawn(move || {
        loop {
            if event::poll(std::time::Duration::from_secs_f64(01. / 30.)).unwrap_or(false) {
                match event::read().unwrap() {
                    Event::Key(key_event) => {
                        let mut s = s1.lock().unwrap();
                        match key_event.code {
                            KeyCode::Char('q') => {
                                s.quit = true;
                                return;
                            } // Exit on 'q'
                            KeyCode::Up => s.change_direction(snake::Direction::U),
                            KeyCode::Down => s.change_direction(snake::Direction::D),
                            KeyCode::Left => s.change_direction(snake::Direction::L),
                            KeyCode::Right => s.change_direction(snake::Direction::R),
                            _ => {} // Handle other keys if needed
                        }
                    }
                    _ => {}
                }
            };
        }
    });
    loop {
        sleep(time::Duration::from_secs_f64(1. / 15.));
        let mut s = s.lock().unwrap();
        if s.quit {
            break;
        }
        s.next();
        let _ = terminal.draw(|f| {
            let b = Block::new().set_style(Style::new().bg(Color::Red));
            f.render_widget(b, Rect::new(s.snake[0].x as u16, s.snake[0].y as u16, 1, 1));
            let b = Block::new().set_style(Style::new().bg(Color::Blue));
            f.render_widget(b, Rect::new(s.food.x as u16, s.food.y as u16, 1, 1));
            let b = Block::new().set_style(Style::new().bg(Color::White));
            for cell in s.snake.clone().iter().skip(1) {
                f.render_widget(&b, Rect::new(cell.x as u16, cell.y as u16, 1, 1));
            }
        });
    }
    t1.join().unwrap();
    ratatui::restore();
    Ok(())
}
