use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::Block,
    Frame,
};
use ratatui_rain::Rain;
use tokio::time;

#[tokio::main]
async fn main() {
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();

    let mut tick_interval = time::interval(time::Duration::from_secs_f64(1.0 / 60.0));

    let start_time = time::Instant::now();

    for _ in 0..600 {
        tick_interval.tick().await;
        terminal
            .draw(|frame| render(frame, start_time.elapsed()))
            .unwrap();
    }

    ratatui::restore();
}

fn render(frame: &mut Frame, elapsed: time::Duration) {
    let [matrix_area, rain_area, snow_area, emoji_area] =
        Layout::horizontal([Constraint::Fill(1); 4]).areas(frame.area());

    render_blocked_rain(frame, Rain::new_matrix(elapsed), " Matrix ", matrix_area);
    render_blocked_rain(frame, Rain::new_rain(elapsed), " Rain ", rain_area);
    render_blocked_rain(frame, Rain::new_snow(elapsed), " Snow ", snow_area);
    // Emoji does leak a bit, since those are multibyte characters.
    render_blocked_rain(frame, Rain::new_emoji_soup(elapsed), " Emoji ", emoji_area);
}

fn render_blocked_rain(frame: &mut Frame, rain: Rain, title: &'static str, area: Rect) {
    let block = Block::bordered().title(title);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);
    frame.render_widget(rain, inner_area);
}
