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
use ratatui::{
    prelude::*,
    widgets::{
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};

use crate::turing_machine::TuringMachine;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum TuiEvent {
    Launch,
    Pause,
    Step,
    Restore,
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
    if event::poll(Duration::from_millis(400))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('l') => return Ok(TuiEvent::Launch),
                KeyCode::Char('p') => return Ok(TuiEvent::Pause),
                KeyCode::Char('s') => return Ok(TuiEvent::Step),
                KeyCode::Char('r') => return Ok(TuiEvent::Restore),
                KeyCode::Char('q') => return Ok(TuiEvent::Quit),
                _ => return Ok(TuiEvent::NoEvent),
            }
        }
    }
    
    Ok(TuiEvent::NoEvent)
}

pub fn ui(frame: &mut Frame, tm: &TuringMachine) {
    const HEIGHT: u16 = 15;
    const WIDTH: u16 = 16 + 36;

    let margin_top = (frame.size().height - HEIGHT) / 2;
    let margin_left = (frame.size().width - WIDTH) / 2;

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(margin_top),
            Constraint::Length(HEIGHT),
            Constraint::Min(0),
        ])
        .split(frame.size());

    let main_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(margin_left),
            Constraint::Length(WIDTH),
            Constraint::Min(0),
        ])
        .split(layout[1]);

    let content = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(9),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(main_area[1]);

    let first_row = content[0];
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(16),
        ])
        .split(content[1]);

    let third_row = content[2];

    render_title(frame, first_row);
    render_info(frame, second_row[0], &tm);
    render_commands(frame, second_row[1]);
    render_tape(frame, third_row, &tm);
}

//fn calculate_layout(area: Rect) -> (Rect, Vec<Vec<Rect>>) {
//    let layout = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(vec![
//            Constraint::Length(1),
//            Constraint::Min(0),
//        ])
//        .split(area);
//    let title_area = layout[0];
//    let main_areas = Layout::default()
//        .direction(Direction::Vertical)
//        .constraints(vec![Constraint::Min(0)])
//        .split(layout[1])
//        .iter()
//        .map(|&area| {
//            Layout::default()
//                .direction(Direction::Horizontal)
//                .constraints(vec![
//                    Constraint::Percentage(80),
//                    Constraint::Percentage(20),
//
//                ])
//                .split(area)
//                .to_vec()
//        })
//        .collect_vec();
//    (title_area, main_areas)
//}

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
        Line::from("l - launch"),
        Line::from("p - pause"),
        Line::from("s - step"),
        Line::from("r - reload"),
        Line::from("q - quit"),
    ];

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area)
}

fn render_info (
    frame: &mut Frame, area: Rect, tm: &TuringMachine
) {
    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("( TMS Info )"))
        .padding(Padding::new(2, 1, 1, 1));

    // Span
    let state_span = Span::styled(
        tm.get_state(),
        Style::default().fg(Color::Green),
    );

    let cell_span = Span::styled(
        tm.get_tape_cell().to_string(),
        Style::default().fg(Color::Green),
    );

    let text = vec![
        Line::from(vec!["State:".white().bold(),  " ".into(), state_span]),
        Line::from(vec!["Cell:".white().bold(), "  ".into(), cell_span]),
    ];

    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(block);

    frame.render_widget(paragraph, area);
}

fn render_tape(
    frame: &mut Frame, area: Rect, tm: &TuringMachine
) {
    let tape = tm.get_tape(); 
    let cell = tm.get_tape_cell();
    let mut skip = if cell == 0 { 1 } else { cell };
    skip *= (skip as u16 / area.width) as usize;
    let len = if tape.len() < area.width as usize {
        tape.len()
    } else {
        area.width as usize
    };
    let mut text = vec![];


    for (i, c) in tape.iter().enumerate().skip(skip.into()).take(len) {
        if i == cell {
            text.push(c.to_string().bold().on_blue().white());
        } else {
            text.push(c.to_string().white());
        }
    }

    let block = Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("( Tape )"))
        .padding(Padding::new(2, 1, 1, 1));

    let paragraph = Paragraph::new(Line::from(text))
        .style(Style::default().fg(Color::Gray))
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap {trim: true});


    frame.render_widget(paragraph, area);
}
