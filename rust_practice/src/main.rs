//Importacions
/*
    std::{io, error::Error}: Importa módulos estándar para manejar entrada/salida y errores.
    crossterm: Proporciona funcionalidades para manejar la terminal, como la entrada de eventos y la manipulación de la pantalla.
    tui: Proporciona herramientas para construir interfaces de usuario en terminal, incluyendo widgets, estilos y layouts.
*/
use std::{io, error::Error};
use crossterm::{
    execute,
    terminal::{self, Clear, ClearType},
    event::{self, KeyCode, KeyEvent, Event},
    cursor::{MoveTo, Hide, Show}
};
use tui::{
    backend::CrosstermBackend, 
    layout::{Constraint, Direction, Layout}, 
    style::{Color, Modifier, Style}, 
    text::{Span, Spans, Text}, 
    widgets::{Block, Borders, List, ListItem, Paragraph}, 
    Frame, Terminal
};
use tui::widgets::ListState;

//Estructura Item
/*
    Item: Estructura que representa un elemento en la lista, con un título y una descripción.
    impl Item: Implementación de métodos para la estructura Item, incluyendo un constructor new.
*/
#[derive(Debug, Clone)]
struct Item {
    title: String,
    description: String,
}

impl Item {
    fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
        }
    }
}

//Estructura App
/*
    App: Estructura que representa el estado de la aplicación, incluyendo una lista de elementos (items), el estado de la lista (state), un indicador para mostrar detalles (show_details), y un indicador para salir (should_quit).
    Default for App: Implementación del trait Default para inicializar la aplicación con algunos elementos predeterminados
*/
struct App {
    items: Vec<Item>,
    state: ListState,
    show_details: bool,
    should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            items: vec![
                Item::new("Victor", "Usuario principal de la aplicación"),
                Item::new("Programación", "Temas relacionados con desarrollo de software"),
                Item::new("Rust", "Lenguaje de programación seguro y concurrente"),
                Item::new("TUI", "Interfaces de usuario en terminal"),
                Item::new("Salir", "Cerrar la aplicación"),
            ],
            state: ListState::default(),
            show_details: false,
            should_quit: false,
        };
        app.state.select(Some(0));
        app
    }
}

//Metodos en App
/*
    next y previous: Métodos para navegar por la lista de elementos.
    on_key: Maneja la entrada de teclas, permitiendo navegar, seleccionar elementos y salir de la aplicación.
    toggle_details: Alterna la visualización de detalles del elemento seleccionado.
    current_item: Devuelve el elemento actualmente seleccionado
*/
impl App {

    //Función siguiente
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    //Fución anterior
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn on_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.previous(),
            KeyCode::Down => self.next(),
            KeyCode::Enter => self.toggle_details(),
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Esc => {
                if self.show_details {
                    self.show_details = false;
                } else {
                    self.should_quit = true;
                }
            }
            _ => {}
        }
    }

    fn toggle_details(&mut self) {
        if let Some(selected) = self.state.selected() {
            if self.items[selected].title == "Salir" {
                self.should_quit = true;
                return;
            }
            self.show_details = !self.show_details;
        }
    }

    fn current_item(&self) -> Option<&Item> {
        self.state.selected().map(|i| &self.items[i])
    }
}

//Funció user interface
/*
    ui: Función que dibuja la interfaz de usuario. Utiliza el marco Frame para renderizar widgets en la terminal.
    Layout: Define cómo se distribuyen los diferentes componentes de la interfaz (título, lista, pie de página).
    Paragraph y List: Widgets que se utilizan para mostrar texto y listas de elementos, respectivamente.
*/

fn ui<B: tui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Título
                Constraint::Min(5),    // Lista
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(f.size());

    // Título
    let title = Paragraph::new(Text::styled(
        "Aplicación TUI con Rust",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(tui::layout::Alignment::Center)
    .block(Block::default().borders(Borders::NONE));
    f.render_widget(title, chunks[0]);

    // Lista principal o detalles
    if app.show_details {
        if let Some(item) = app.current_item() {
            let detail_text = Text::from(vec![
                Spans::from(vec![Span::styled(
                    format!("{}\n", item.title),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )]),
                Spans::from(vec![Span::raw(format!("{}\n\n", item.description))]),
                Spans::from(vec![Span::raw("Presiona ESC para volver atrás")]),
            ]);
            
            let details = Paragraph::new(detail_text)
                .block(
                    Block::default()
                        .title("Detalles")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightGreen)),
                )
                .alignment(tui::layout::Alignment::Left);
            f.render_widget(details, chunks[1]);
        }
    } else {
        let items: Vec<ListItem> = app
            .items
            .iter()
            .map(|i| {
                let content = Spans::from(vec![Span::raw(&i.title)]);
                ListItem::new(content).style(Style::default().fg(Color::White))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Menú principal")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightBlue)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, chunks[1], &mut app.state);
    }

    // Footer
    let footer = Paragraph::new(Text::styled(
        "↑/↓: Navegar | Enter: Seleccionar | q/ESC: Salir",
        Style::default().fg(Color::Gray),
    ))
    .alignment(tui::layout::Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

//Función para comenzar el programa
fn startup() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    execute!(
        io::stdout(),
        terminal::EnterAlternateScreen,
        Hide,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    Ok(())
}

//Función para parar el programa
fn shutdown() -> Result<(), Box<dyn Error>> {
    execute!(
        io::stdout(),
        Show,
        terminal::LeaveAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}


//Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configuración inicial
    startup()?;
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Estado de la aplicación
    let mut app = App::default();

    // Bucle principal
    while !app.should_quit {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key);
            }
        }
    }

    // Limpieza al salir
    shutdown()?;
    Ok(())
}