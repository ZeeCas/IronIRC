use ironirc::app::{App, AppResult};
use ironirc::event::{Event, EventHandler};
use ironirc::handler::handle_key_events;
use ironirc::tui::Tui;
use std::io;
use std::path;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use irc::client::prelude::*;




#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let config = Config::load(path::Path::new("config.toml")).unwrap();
    let mut client = Client::from_config(config).await?;
    client.identify()?;
    let stream = client.stream()?;

    let mut app = App {
        running: true,
        input: String::new(),
        cursor_position: 0,
        messages: vec![],
        client: Some(client),
        stream: Some(stream),
        vertical_scroll: 0,
        horizontal_scroll: 0,
        vertical_scroll_state: Default::default(),
        horizontal_scroll_state: Default::default(),

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
