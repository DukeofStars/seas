use std::{
    io,
    sync::{Arc, Mutex},
};

use crate::share::Global;

// Until ui properly implemented this will do nothing.
pub async fn run_ui(globals: Arc<Mutex<Global>>) {
    while !globals.lock().unwrap().should_close {}
}

// pub async fn run_ui(globals: Arc<Mutex<Global>>) {
//     // setup terminal
//     enable_raw_mode().unwrap();
//     let mut stdout = std::io::stdout();
//     execute!(stdout, Clear(ClearType::All), EnableMouseCapture).unwrap();
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend).unwrap();
//
//     loop {
//         if globals.lock().unwrap().should_close {
//             break;
//         }
//         terminal
//             .draw(|f| {
//                 let mut size = f.size();
//                 size.height = 5;
//                 let chunks = Layout::default()
//                     .direction(Direction::Vertical)
//                     .constraints([])
//                     .split(f.size());
//
//                 let block = Block::default()
//                     .title(Span::styled("Tui", Style::default().fg(Color::Blue)))
//                     .border_style(Style::default().fg(Color::Green))
//                     .border_type(BorderType::Rounded)
//                     .borders(Borders::ALL);
//
//                 let pb = LineGauge::default()
//                     //.block(block)
//                     .gauge_style(
//                         Style::default()
//                             .fg(Color::Green)
//                             .bg(Color::Red)
//                             .add_modifier(Modifier::BOLD),
//                     )
//                     .line_set(symbols::line::THICK)
//                     .ratio(globals.lock().unwrap().percentage.clamp(0., 1.)); // Prevent floating point errors over 1.0 which can crash the tui
//                 f.render_widget(pb, chunks[0]);
//                 f.render_widget(Block::default().title("My Title"), chunks[1]);
//             })
//             .unwrap();
//     }
//
//     // restore terminal
//     disable_raw_mode().unwrap();
//     execute!(terminal.backend_mut(), DisableMouseCapture).unwrap();
//     terminal.show_cursor().unwrap();
// }
