mod app;
mod types;
mod ui;

use app::App;

fn main() {
    let terminal = ratatui::init();
    let app = App::create();
    app.run(terminal);
    ratatui::restore();
}
