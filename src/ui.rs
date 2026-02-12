use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};
use std::path::Path;
use std::time::Duration;

use crate::player::PlaybackState;

pub struct UIState {
    pub filename: String,
    pub position: Duration,
    pub duration: Duration,
    pub volume: f32,
    pub state: PlaybackState,
    pub waveform: Vec<f32>,
}

impl UIState {
    pub fn new<P: AsRef<Path>>(path: P, duration: Duration, waveform: Vec<f32>) -> Self {
        let filename = path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        Self {
            filename,
            position: Duration::from_secs(0),
            duration,
            volume: 1.0,
            state: PlaybackState::Paused,
            waveform,
        }
    }
}

pub fn render(frame: &mut Frame, state: &UIState) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3),  // Title
        Constraint::Length(5),  // Waveform
        Constraint::Length(3),  // Progress
        Constraint::Length(3),  // Volume
        Constraint::Min(0),     // Spacer
        Constraint::Length(3),  // Controls
    ])
    .split(area);

    render_title(frame, chunks[0], state);
    render_waveform(frame, chunks[1], state);
    render_progress(frame, chunks[2], state);
    render_volume(frame, chunks[3], state);
    render_controls(frame, chunks[5]);
}

fn render_waveform(frame: &mut Frame, area: Rect, state: &UIState) {
    let width = area.width.saturating_sub(2) as usize;
    let waveform_data: Vec<u64> = if state.waveform.len() >= width {
        state.waveform[..width]
            .iter()
            .map(|&v| (v * 100.0) as u64)
            .collect()
    } else {
        let scale = width as f32 / state.waveform.len() as f32;
        (0..width)
            .map(|i| {
                let idx = (i as f32 / scale) as usize;
                if idx < state.waveform.len() {
                    (state.waveform[idx] * 100.0) as u64
                } else {
                    0
                }
            })
            .collect()
    };

    let position_secs = state.position.as_secs();
    let duration_secs = state.duration.as_secs().max(1);
    let progress_ratio = position_secs as f64 / duration_secs as f64;
    let _cursor_pos = (progress_ratio * width as f64) as usize;

    let waveform_color = match state.state {
        PlaybackState::Playing => Color::Cyan,
        PlaybackState::Paused => Color::Yellow,
        PlaybackState::Stopped => Color::DarkGray,
    };

    let sparkline = Sparkline::default()
        .block(Block::default().borders(Borders::ALL).title("Waveform"))
        .data(&waveform_data)
        .style(Style::default().fg(waveform_color));

    frame.render_widget(sparkline, area);
}

fn render_title(frame: &mut Frame, area: Rect, state: &UIState) {
    let status_symbol = match state.state {
        PlaybackState::Playing => "▶",
        PlaybackState::Paused => "⏸",
        PlaybackState::Stopped => "⏹",
    };

    let status_color = match state.state {
        PlaybackState::Playing => Color::Green,
        PlaybackState::Paused => Color::Yellow,
        PlaybackState::Stopped => Color::Red,
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            status_symbol,
            Style::default().fg(status_color).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            &state.filename,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "aud",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )),
    );

    frame.render_widget(title, area);
}

fn render_progress(frame: &mut Frame, area: Rect, state: &UIState) {
    let position_secs = state.position.as_secs();
    let duration_secs = state.duration.as_secs().max(1);
    let ratio = (position_secs as f64 / duration_secs as f64).min(1.0);

    let position_str = format_duration(state.position);
    let duration_str = format_duration(state.duration);
    let label = format!("{} / {}", position_str, duration_str);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .label(label)
        .ratio(ratio);

    frame.render_widget(gauge, area);
}

fn render_volume(frame: &mut Frame, area: Rect, state: &UIState) {
    let volume_percent = (state.volume * 100.0) as u16;
    let label = format!("{}%", volume_percent);

    let volume_color = if state.volume > 0.7 {
        Color::Green
    } else if state.volume > 0.3 {
        Color::Yellow
    } else {
        Color::Red
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Volume"))
        .gauge_style(Style::default().fg(volume_color).bg(Color::DarkGray))
        .label(label)
        .ratio(state.volume as f64);

    frame.render_widget(gauge, area);
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let controls = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("[Space]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" play/pause  "),
            Span::styled("[Q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" quit  "),
            Span::styled("[R]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" restart"),
        ]),
        Line::from(vec![
            Span::styled("[←/→]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" seek ±5s  "),
            Span::styled("[↑/↓]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(" volume ±5%"),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Controls"));

    frame.render_widget(controls, area);
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
