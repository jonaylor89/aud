use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

use crate::player::Player;

pub enum ControlAction {
    Quit,
    Continue,
}

pub fn handle_input(player: &Player) -> Result<ControlAction, Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(100))?
        && let Event::Key(KeyEvent { code, .. }) = event::read()?
    {
        match code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                return Ok(ControlAction::Quit);
            }
            KeyCode::Char(' ') => {
                player.toggle_play_pause();
            }
            KeyCode::Left => {
                player.seek(-player.seek_step);
            }
            KeyCode::Right => {
                player.seek(player.seek_step);
            }
            KeyCode::Up => {
                let new_volume = (player.volume() + player.volume_step).min(1.0);
                player.set_volume(new_volume);
            }
            KeyCode::Down => {
                let new_volume = (player.volume() - player.volume_step).max(0.0);
                player.set_volume(new_volume);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                player.restart();
            }
            _ => {}
        }
    }

    Ok(ControlAction::Continue)
}
