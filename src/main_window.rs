use fltk::{prelude::*, *};

/// A window with a canvas and color picker
pub struct MainWindow {
    /// The actual window widget
    inner: window::DoubleWindow,
}

impl MainWindow {
    // Creates a new main window
    pub fn new(width: i32, height: i32) -> Self {
        // Creates the main window
        let inner = window::DoubleWindow::default().with_size(width, height);
        MainWindow { inner }
    }
}

fltk::widget_extends!(MainWindow, window::DoubleWindow, inner);
