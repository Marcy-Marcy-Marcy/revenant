mod main_window;
use fltk::{app, prelude::WidgetExt};
use main_window::MainWindow;

fn main() {
    let app = app::App::default();
    let mut window = MainWindow::new(400, 400);
    window.show();
    app.run().expect("The application should be able to run")
}
