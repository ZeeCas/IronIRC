use std::error;
use futures::prelude::*;

use ratatui::widgets::ScrollbarState;
use tokio::time::timeout;
use std::time::Duration;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub messages: Vec<String>,

    pub client: Option<irc::client::Client>,

    pub stream: Option<irc::client::ClientStream>,

    pub input: String,

    pub cursor_position: usize,

    pub vertical_scroll: u16,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll: u16,
    pub horizontal_scroll_state: ScrollbarState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            messages: Vec::new(),
            client: None,
            stream: None,
            input: String::new(),
            cursor_position: 0,
            vertical_scroll: 0,
            horizontal_scroll: 0,
            vertical_scroll_state: Default::default(),
            horizontal_scroll_state: Default::default(),
        }
    }
}
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub async fn tick(&mut self) {
        if let Ok(Some(Ok(message))) = timeout(Duration::from_millis(5), self.stream.as_mut().unwrap().next()).await {
            self.messages.push(message.to_string());
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.client.as_mut().unwrap().send_quit("Goodbye").unwrap();
        self.running = false;
    }

    pub fn send_message(&mut self, message: String) {
        let _result = self.client.as_mut().unwrap().send_privmsg("#main", message.clone());
        self.input.clear();
        self.reset_cursor();
        self.messages.push("You: ".to_owned() + &message + "\n");
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}
