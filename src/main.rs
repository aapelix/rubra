use gtk4::{Application, ApplicationWindow, Notebook, Settings};
use gtk4::prelude::*;
use tab::create_tab;

mod tab;
mod setting;
mod search;

fn main() {
    let app = Application::builder()
        .application_id("dev.aapelix.rubra")
        .build();

    app.connect_activate(|app| {
       create_window(&app);
    });

    app.run();
}

fn create_window(app: &Application) {

    let settings: Settings = Settings::default().unwrap();

    settings.set_gtk_application_prefer_dark_theme(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1500)
        .default_height(900)
        .title("aapelix/rubra")
        .resizable(true)
        .build();

    let notebook = Notebook::new();
    notebook.set_scrollable(true);

    create_tab("https://start.duckduckgo.com/", &notebook, &app);

    window.set_child(Some(&notebook));

    window.present();
}
