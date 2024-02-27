use std::{error, vec};
use futures::prelude::*;
use irc::proto::message;

use std::collections::HashMap as Hashmap;
use ratatui::widgets::ScrollbarState;
use tokio::time::timeout;
use std::time::Duration;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub enum Mode {
    Normal,
    Command
}

#[derive(Debug)]
pub struct App{
    /// Is the application running?
    pub running: bool,
    pub mode: Mode,
    pub show_users: bool,
    pub selected_tab: usize,
    pub num_tabs: usize,
    pub tab_titles: Vec<String>,


    pub messages: Hashmap<String, Vec<String>>,
    pub input: String,

    pub client: Option<irc::client::Client>,
    pub stream: Option<irc::client::ClientStream>,
    pub active_channel: String,
    pub active_channel_users: Vec<String>,    

    pub cursor_position: usize,
    pub vertical_scroll: Vec<u16>,
    pub vertical_scroll_state: Vec<ScrollbarState>,
    pub horizontal_scroll: Vec<u16>,
    pub horizontal_scroll_state: Vec<ScrollbarState>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            messages: Hashmap::new(),
            client: None,
            stream: None,
            active_channel: "".to_string(),
            active_channel_users: Vec::new(),
            show_users: true,
            selected_tab: 0,
            num_tabs: 1,
            tab_titles: vec![],
            input: String::new(),
            cursor_position: 0,
            vertical_scroll: vec![],
            horizontal_scroll: vec![],
            vertical_scroll_state: vec![],
            horizontal_scroll_state: vec![],
            mode: Mode::Normal
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
            if let Some(channel) = message.response_target() {
                let message_content = message.to_string();
                self.messages.entry(channel.to_string()).or_default().push(message_content);
            } else {
                let message_content = message.to_string();
                self.messages.entry("!server".to_string()).or_default().push(message_content);
            }
        }
        if let Some(_users) = self.client.as_mut().unwrap().list_users(&self.active_channel) {
            self.get_active_channel_users();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.client.as_mut().unwrap().send_quit("Goodbye").unwrap();
        self.running = false;
    }

    pub fn get_input(&mut self) -> Option<String> {
        let output = Some(self.input.clone());
        self.input.clear();
        self.reset_cursor();
        output
    }

    pub fn send_message(&mut self) {
        let message = self.get_input().unwrap();
        let _result = self.client.as_mut().unwrap().send_privmsg(self.active_channel.clone(), message.clone());
        self.messages.entry(self.active_channel.clone()).or_default().push("You: ".to_string() + message.as_str() + "\n");
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

    pub fn process_command(&mut self) {
        let command = self.get_input().unwrap();
        let command_vec: Vec<&str> = command.split_whitespace().collect();
        match command_vec[0] {
            "quit" | "q" => self.quit(),
            "join" | "j "=> {
                self.client.as_mut().unwrap().send_join(command_vec[1]).unwrap();
                self.tab_titles.push(command_vec[1].to_string());
                self.active_channel = command_vec[1].to_string();
            },
            "users" | "u" => {
                self.show_users = !self.show_users;
            }

            _ => {}
        }
    }

    pub fn get_active_channel_users(&mut self) {
        self.active_channel_users.clear();
        let users = self.client.as_ref().unwrap().list_users(self.active_channel.clone().as_str()).unwrap();
        for user in users {
            self.active_channel_users.push(user.get_nickname().to_string());
        }
    }

    pub fn next_tab(&mut self) {
        if self.selected_tab.saturating_add(1) < self.tab_titles.len() {
            self.selected_tab += 1;
        }
        self.active_channel = self.tab_titles[self.selected_tab].clone();
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = self.selected_tab.saturating_sub(1);
        self.active_channel = self.tab_titles[self.selected_tab].clone();
    }
}
