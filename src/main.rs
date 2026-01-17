#[allow(dead_code)]
mod app;
mod types;
mod ui;

use app::App;

fn main() {
    let app = App::create();

    for todo in app.todos.items() {
        println!("{}", todo.pretty());
    }
}
