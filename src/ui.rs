use std::time::Instant;

use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{self, Line, Span},
    widgets::{Block, Padding, Paragraph, Sparkline, Tabs},
    Frame,
};
use systemstat::Platform;

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ])
    .margin(1)
    .split(f.size());

    // Tabs
    draw_tabs(f, app, chunks[0]);
    match app.tabs.index {
        0 => draw_info_tab(f, app, chunks[1]),
        1 => draw_queue_tab(f, app, chunks[1]),
        2 => draw_settings_tab(f, app, chunks[1]),
        _ => {}
    }

    // Footer
    f.render_widget(
        Line::raw("◄ ► to change tab | Press q to quit")
            .centered()
            .light_cyan(),
        chunks[2],
    );
}

fn draw_tabs(f: &mut Frame, app: &mut App, area: Rect) {
    f.render_widget(
        app.tabs
            .titles
            .iter()
            .map(|t| text::Line::from(Span::styled(*t, Style::default())))
            .collect::<Tabs>()
            .block(
                block()
                    .border_style(Style::default().fg(Color::LightCyan))
                    .title(format!("Andinum POD v{}", app.version))
                    .title_style(Color::White)
                    .title_alignment(Alignment::Center),
            )
            .highlight_style(Style::default().fg(Color::DarkGray))
            .select(app.tabs.index)
            .padding("", "")
            .divider(" "),
        area,
    )
}

fn draw_info_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let vertical = Layout::vertical([Constraint::Fill(1), Constraint::Fill(4)]).split(area);
    let horizontal = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical[0]);

    if app.info_stats.last_update.elapsed().as_secs() >= 15 {
        app.info_stats.last_update = Instant::now();
        let sys = &app.info_stats.sys;

        if let Ok(cpu) = sys.cpu_load_aggregate() {
            let cpu = cpu.done().unwrap();
            let cpu_usage = ((cpu.user * 100.0) + (cpu.system * 100.0)) as u64;
            if cpu_usage > 0 && cpu_usage < 100 {
                app.info_stats.cpu_usage.push(cpu_usage);
            }
        }

        if let Ok(mem) = sys.memory() {
            let mem_usage = (mem.total.as_u64() - mem.free.as_u64()) * 100 / mem.total.as_u64();
            if mem_usage > 0 && mem_usage < 100 {
                app.info_stats.mem_usage.push(mem_usage);
            }
        }
    }

    f.render_widget(
        Sparkline::default()
            .block(block().title("CPU Usage").title_style(Color::White))
            .data(&app.info_stats.cpu_usage)
            .style(Style::default().fg(Color::LightYellow)),
        horizontal[0],
    );
    f.render_widget(
        Sparkline::default()
            .block(block().title("Memory Usage").title_style(Color::White))
            .data(&app.info_stats.mem_usage)
            .style(Style::default().fg(Color::LightYellow)),
        horizontal[1],
    );
}

fn draw_queue_tab(f: &mut Frame, _app: &mut App, area: Rect) {
    let p = Paragraph::new("Hi").block(block());

    f.render_widget(p, area);
}

fn draw_settings_tab(f: &mut Frame, _app: &mut App, area: Rect) {
    let p = Paragraph::new("Hi").block(block());

    f.render_widget(p, area);
}

fn block<'static_>() -> Block<'static_> {
    Block::bordered()
        .padding(Padding::horizontal(1))
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(Color::LightGreen))
}

pub struct InfoStats {
    pub last_update: Instant,
    pub sys: systemstat::System,

    pub cpu_usage: Vec<u64>,
    pub mem_usage: Vec<u64>,
}

impl InfoStats {
    pub fn new(sys: systemstat::System) -> Self {
        Self {
            last_update: Instant::now(),
            sys,

            cpu_usage: Vec::new(),
            mem_usage: Vec::new(),
        }
    }
}
