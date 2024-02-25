use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Enter => {
            app.send_message(app.input.clone());
        }
        KeyCode::Up => {
            app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll as usize);
        }
        KeyCode::Down => {
            app.vertical_scroll = app.vertical_scroll.saturating_add(1);
            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll as usize);
        }

        KeyCode::Char(c) => {
            app.enter_char(c);
        }
        _ => {}
    }
    Ok(())
}
