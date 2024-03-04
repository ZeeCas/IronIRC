use crate::app::{App, AppResult, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.mode {
        Mode::Normal => match key_event {
            KeyEvent {code: KeyCode::Insert, ..} => {
                app.mode = Mode::Command;
            }
            // Exit application on `ESC`
            KeyEvent {code: KeyCode::Esc, ..} => {
                app.quit();
            }
            KeyEvent {code: KeyCode::Backspace, ..} => {
                app.delete_char();
            }
            KeyEvent {code: KeyCode::Left, ..} => {
                app.move_cursor_left();
            }
            KeyEvent {code: KeyCode::Right, ..} => {
                app.move_cursor_right();
            }
            KeyEvent {code: KeyCode::Enter, ..} => {
                app.send_message();
            }
            KeyEvent {code: KeyCode::Up, ..} => {
                app.vertical_scroll[app.selected_tab] = app.vertical_scroll[app.selected_tab].saturating_sub(1);
                app.vertical_scroll_state[app.selected_tab] = app.vertical_scroll_state[app.selected_tab].position(app.vertical_scroll[app.selected_tab] as usize);
            }
            KeyEvent {code: KeyCode::Down, ..} => {
                app.vertical_scroll[app.selected_tab] = app.vertical_scroll[app.selected_tab].saturating_add(1);
                app.vertical_scroll_state[app.selected_tab] = app.vertical_scroll_state[app.selected_tab].position(app.vertical_scroll[app.selected_tab] as usize);
            }
    
            KeyEvent {code: KeyCode::Char(c), ..} => {
                app.enter_char(c);
            }
            _ => {}
        },
        Mode::Command => match key_event {
            KeyEvent {code: KeyCode::Esc, modifiers: KeyModifiers::NONE, ..} => {
                app.mode = Mode::Normal;
            }
            KeyEvent {code: KeyCode::Insert, ..} => {
                app.mode = Mode::Normal;
            }
            KeyEvent {code: KeyCode::Char(c), ..} => {
                app.enter_char(c);
            }
            KeyEvent {code: KeyCode::Backspace, ..} => {
                app.delete_char();
            }
            KeyEvent {code: KeyCode::Left, ..} => {
                app.prev_tab();
            }
            KeyEvent {code: KeyCode::Right, ..} => {
                app.next_tab();
            }
            KeyEvent {code: KeyCode::Enter, ..} => {
                app.process_command();
            }
            KeyEvent {code: KeyCode::Up, ..} => {
                app.vertical_scroll[app.selected_tab] = app.vertical_scroll[app.selected_tab].saturating_sub(1);
                app.vertical_scroll_state[app.selected_tab] = app.vertical_scroll_state[app.selected_tab].position(app.vertical_scroll[app.selected_tab] as usize);
            }
            KeyEvent {code: KeyCode::Down, ..} => {
                app.vertical_scroll[app.selected_tab] = app.vertical_scroll[app.selected_tab].saturating_add(1);
                app.vertical_scroll_state[app.selected_tab] = app.vertical_scroll_state[app.selected_tab].position(app.vertical_scroll[app.selected_tab] as usize);
            }   
            _ => {}
        },
    }
    
    Ok(())
}
