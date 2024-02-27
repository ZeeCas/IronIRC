use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs, Wrap},
    Frame,
};

use std::collections::HashMap as Hashmap;

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let messages = app.messages.clone();
    let mut tabs:Hashmap<String, Paragraph> = Hashmap::new();
    for title in &app.tab_titles {
        match messages.get(title) {
            None => {}
            Some(_) => {
                if app.vertical_scroll.get(app.selected_tab).is_none() {
                    app.vertical_scroll.push(0);
                    app.vertical_scroll_state.push(Default::default());
                };
                if app.horizontal_scroll.get(app.selected_tab).is_none() {
                    app.horizontal_scroll.push(0);
                    app.horizontal_scroll_state.push(Default::default());
                };
                tabs.insert(title.clone(), Paragraph::new(messages.get(title).unwrap().join(""))
                                                            .block(
                                                                Block::default()
                                                                    .borders(Borders::ALL)
                                                                    .border_type(BorderType::Rounded)
                                                                    .title(title.to_string())
                                                                    .title_style(Style::default().fg(Color::Yellow))
                                                                    .style(Style::default().fg(Color::White)),
                                                            )
                                                            .alignment(Alignment::Left)
                                                            .scroll((app.vertical_scroll[app.selected_tab], app.horizontal_scroll[app.selected_tab]))
                                                            .wrap(Wrap { trim: false }));
                                                        app.vertical_scroll_state[app.selected_tab] = app.vertical_scroll_state[app.selected_tab].content_length(messages.get(title).unwrap().len());
                                                        app.horizontal_scroll_state[app.selected_tab] = app.horizontal_scroll_state[app.selected_tab].content_length(messages.get(title).unwrap().len());
                            }
        }
    }
    let chunks = Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).split(frame.size());
    let vert_chunks = Layout::horizontal([Constraint::Percentage(90), Constraint::Percentage(10)]).split(chunks[0]);
    let tab_chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(vert_chunks[0]);
    let tabs_list = Tabs::new(app.tab_titles.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Tabs")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)),
        ).highlight_style(Style::default().fg(Color::Yellow))
        .select(app.selected_tab);
    // let main = Paragraph::new("")
        // .block(
        //     Block::default()
        //         .borders(Borders::ALL)
        //         .border_type(BorderType::Rounded)
        //         .title("Messages")
        //         .title_style(Style::default().fg(Color::Yellow))
        //         .style(Style::default().fg(Color::White)),
        // )
        // .alignment(Alignment::Left)
        // .scroll((app.vertical_scroll, app.horizontal_scroll))
        // .wrap(Wrap { trim: false });
    let users = Paragraph::new(app.active_channel_users.join("\n"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Users")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)),
        ).alignment(Alignment::Left);
    let input_box = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Input")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)),
        ).alignment(Alignment::Left);
    let command_box = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Command")
            .title_style(Style::default().fg(Color::Yellow))
            .style(Style::default().fg(Color::White)),
        ).alignment(Alignment::Left);
            
    frame.render_widget(tabs_list, tab_chunks[0]);
    if let Some(tab) = tabs.get(&app.tab_titles[app.selected_tab]) {
        frame.render_widget(tab, tab_chunks[1]);
        frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), tab_chunks[1].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        }), &mut app.vertical_scroll_state[app.selected_tab]);
    }
    match app.mode {
        crate::app::Mode::Normal => frame.render_widget(input_box, chunks[1]),
        crate::app::Mode::Command => frame.render_widget(command_box, chunks[1]),
    }
    match app.show_users {
        true => frame.render_widget(users, vert_chunks[1]),
        false => {}
    }
    frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), tab_chunks[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    }), &mut app.vertical_scroll_state[0])
}
