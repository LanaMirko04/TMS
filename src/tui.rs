use std::io;
use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};

use crate::turing_machine::TuringMachine;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum TuiEvent {
    Run,
    Pause,
    Step,
    Reset,
    Quit,
    NoEvent,
}

pub fn setup_terminal() -> Result<Terminal, io::Error> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(mut terminal: Terminal) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn handle_events() -> Result<TuiEvent, io::Error> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(TuiEvent::Quit);
            }
        }
    }
    Ok(TuiEvent::NoEvent)
}

pub fn ui(frame: &mut Frame, tm: &TuringMachine) {
    let (title_area, layout) = calculate_layout(frame.size());

    render_title(frame, title_area);
    render_commands(frame, layout[0][1]);
}

fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);
    let title_area = layout[0];
    let main_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(0)])
        .split(layout[1])
        .iter()
        .map(|&area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(80),
                    Constraint::Percentage(20),

                ])
                .split(area)
                .to_vec()
        })
        .collect_vec();
    (title_area, main_areas)
}

fn render_title(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Turing Machine Simulator")
            .dark_gray()
            .alignment(Alignment::Center),
        area,
    );
}

fn render_commands(frame: &mut Frame, area: Rect) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("( Commands )"))
        .padding(Padding::new(2, 1, 1, 1));
    
    let text = vec![
        Line::from("r - run"),
        Line::from("p - pause"),
        Line::from("s - step"),
        Line::from("r - reset"),
        Line::from("q - quit"),
    ];

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area)
}

fn render_tape(frame: &mut Frame, area: Rect, tm: &TuringMachine) {
    //frame.render_widget()
}
