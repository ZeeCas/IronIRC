use ratatui::{
    layout::{Layout, Alignment, Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let messages = app.messages.clone();
    app.vertical_scroll_state = app.vertical_scroll_state.content_length(messages.len());
    app.horizontal_scroll_state = app.horizontal_scroll_state.content_length(messages.len());
    let chunks = Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]).split(frame.size());
    let main = Paragraph::new(messages.join(" "))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Messages")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Left)
        .scroll((app.vertical_scroll, app.horizontal_scroll))
        .wrap(Wrap { trim: false });
    let input_box = Paragraph::new(app.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Input")
                .title_style(Style::default().fg(Color::Yellow))
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Left);
    frame.render_widget(main, chunks[0]);
    frame.render_widget(input_box, chunks[1]);
    frame.render_stateful_widget(Scrollbar::new(ScrollbarOrientation::VerticalRight), chunks[0], &mut app.vertical_scroll_state)
}
