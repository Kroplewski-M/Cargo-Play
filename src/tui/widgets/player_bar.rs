use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Gauge, Paragraph},
};

use crate::{
    MAIN_COLOR,
    app::AppState,
    tui::{
        state::{Focus, UiState},
        widgets::Section,
    },
};

const SECTION: Section = Section(Focus::PlayerBar);

pub fn render(frame: &mut Frame, area: Rect, app: &AppState, ui: &mut UiState) {
    let block = SECTION.block(" Player ", ui);
    let inner = block.inner(area);

    let [track_row, controls_row, progress_row, _gap, volume_row] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    // Track name
    let (track_text, track_style) = match &app.player_control.current_track {
        Some(t) => (
            t.name.as_str(),
            Style::new().fg(Color::White).add_modifier(Modifier::BOLD),
        ),
        None => ("No track selected", Style::new().fg(Color::DarkGray)),
    };
    frame.render_widget(
        Paragraph::new(track_text)
            .alignment(Alignment::Center)
            .style(track_style),
        track_row,
    );

    let play_icon = if app.player_control.current_track.is_some() && !app.player_control.is_paused()
    {
        "⏸"
    } else {
        "▶"
    };
    frame.render_widget(
        Paragraph::new(format!("  ⏮   {}   ⏭  ", play_icon))
            .alignment(Alignment::Center)
            .style(Style::new().fg(MAIN_COLOR)),
        controls_row,
    );

    // Progress bar
    if let Some(track) = &app.player_control.current_track {
        let ratio = if track.duration > 0.0 {
            (app.player_control.track_position_secs() / track.duration as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let [time_left, bar, time_right] = Layout::horizontal([
            Constraint::Length(6),
            Constraint::Min(0),
            Constraint::Length(9),
        ])
        .areas(progress_row);

        frame.render_widget(
            Paragraph::new(format!(
                "{} ",
                app.player_control.track_formatted_position()
            ))
            .alignment(Alignment::Right)
            .style(Style::new().fg(Color::Gray)),
            time_left,
        );
        frame.render_widget(
            Gauge::default()
                .gauge_style(Style::new().fg(MAIN_COLOR).bg(Color::DarkGray))
                .use_unicode(true)
                .ratio(ratio)
                .label(""),
            bar,
        );
        frame.render_widget(
            Paragraph::new(format!(" {}", track.formatted_duration()))
                .alignment(Alignment::Left)
                .style(Style::new().fg(Color::Gray)),
            time_right,
        );
    } else {
        frame.render_widget(
            Gauge::default()
                .gauge_style(Style::new().fg(Color::DarkGray))
                .ratio(0.0)
                .label(""),
            progress_row,
        );
    }

    // Volume bar
    let [_spacer, vol_section] =
        Layout::horizontal([Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)]).areas(volume_row);

    let vol = app.player_control.volume();
    let [vol_label, vol_bar, vol_pct] = Layout::horizontal([
        Constraint::Length(5),
        Constraint::Min(0),
        Constraint::Length(5),
    ])
    .areas(vol_section);

    frame.render_widget(
        Paragraph::new(" Vol").style(Style::new().fg(Color::Gray)),
        vol_label,
    );
    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::new().fg(Color::Yellow).bg(Color::DarkGray))
            .use_unicode(true)
            .ratio(vol as f64)
            .label(""),
        vol_bar,
    );
    frame.render_widget(
        Paragraph::new(format!("{:3}%", (vol * 100.0) as u32))
            .alignment(Alignment::Right)
            .style(Style::new().fg(Color::Gray)),
        vol_pct,
    );

    frame.render_widget(block, area);
}
