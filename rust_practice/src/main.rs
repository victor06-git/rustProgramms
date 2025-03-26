use std::io;
use crossterm::{execute, terminal};
use crossterm::event::{self, KeyCode, KeyEvent};
use tui::{backend::CrosstermBackend, widgets::{Block, Borders, List, ListItem, Paragraph}, Terminal, widgets::ListState};

#[derive(Default)]
struct App {
    items: Vec<String>,
    state: ListState,
}

impl App {
    fn next(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected < self.items.len() - 1 {
                self.state.select(Some(selected + 1));
            }
        } else {
            self.state.select(Some(0));
        }
    }

    fn previous(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected > 0 {
                self.state.select(Some(selected - 1));
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App {
        items: vec!["Victor".to_string(), "Programación".to_string(), "Item 3".to_string()],
        state: ListState::default(),
    };

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<ListItem> = app.items.iter().map(|i| ListItem::new(i.clone())).collect();
            let list = List::new(items)
                .block(Block::default().title("Items").borders(Borders::ALL))
                .highlight_symbol(">> ")
                .highlight_style(tui::style::Style::default().fg(tui::style::Color::Yellow));
            
            f.render_stateful_widget(list, size, &mut app.state);
        })?;

        if event::poll(std::time::Duration::from_millis(500))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Enter => {
                        if let Some(selected) = app.state.selected() {
                            // Aquí puedes implementar la lógica para entrar en el ítem seleccionado
                            println!("Has seleccionado: {}", app.items[selected]);
                            // Por ejemplo, mostrar detalles del ítem
                        }
                    }
                    KeyCode::Esc => break, // Salir al presionar Esc
                    _ => {}
                }
            }
        }
    }

    terminal.clear()?;
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), terminal::LeaveAlternateScreen)?;
    Ok(())
}