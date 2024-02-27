use ironirc::app::{App, AppResult, Mode};
use ironirc::event::{Event, EventHandler};
use ironirc::handler::handle_key_events;
use ironirc::tui::Tui;
use std::collections::HashMap;
use std::{io, vec};
use std::path;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use irc::client::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let config = Config::load(path::Path::new("config.toml")).unwrap();
    let mut client = Client::from_config(config).await?;
    let channels = vec!["!server".to_string(), "#client-testing".to_string()];
    client.send_cap_req(&[Capability::MultiPrefix])?;
    client.send_sasl_plain()?;
    client.identify()?;
    let stream = client.stream()?;

    let mut app = App {
        running: true,
        input: String::new(),
        cursor_position: 0,
        messages: HashMap::new(),
        client: Some(client),
        stream: Some(stream),
        active_channel: channels[0].clone(),
        active_channel_users: vec![],
        vertical_scroll: vec![0],
        horizontal_scroll: vec![0],
        vertical_scroll_state: vec![Default::default()],
        horizontal_scroll_state: vec![Default::default()],
        mode: Mode::Normal,
        show_users: true,
        selected_tab: 0,
        num_tabs: 1,
        tab_titles: channels,
    };

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Tick => app.tick().await,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

