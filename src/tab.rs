use gtk4::{Application, Button, Entry};
use gtk4::{prelude::*, Box, Label, Notebook};
use webkit6::{CookieManager, NetworkSession, WebView};
use webkit6::prelude::*;

use crate::search::process_search_input;
use crate::setting::{create_settings_window, load_settings, apply_settings};

pub fn create_tab(default_uri: &str, notebook: &Notebook, app: &Application) {
    let tab_box = Box::new(gtk4::Orientation::Horizontal, 0);
    let tab_label = Label::new(Some("New label"));
    let tab_close = Button::with_label("x");

    tab_box.append(&tab_label);
    tab_box.append(&tab_close);

    let hbox = Box::new(gtk4::Orientation::Vertical, 0);
    let top_bar = Box::new(gtk4::Orientation::Horizontal, 0);

    let back = Button::with_label("<");
    let forward = Button::with_label(">");
    let refresh = Button::with_label("⟳");

    top_bar.append(&back);
    top_bar.append(&forward);
    top_bar.append(&refresh);

    let search_e = Entry::new();
    search_e.set_halign(gtk4::Align::Fill);
    search_e.set_hexpand(true);

    top_bar.append(&search_e);

    let new = Button::with_label("+");
    let settings = Button::with_label("⋮");

    top_bar.append(&new);
    top_bar.append(&settings);

    let settings_rc = load_settings();

    // Create and configure WebView
    let webview = WebView::new();

    let ses: NetworkSession = webview.network_session().expect("no network session");

    let cookie: CookieManager = ses.cookie_manager().expect("cookie manager not found");

    cookie.set_persistent_storage("rubra-cookies.sqlite", webkit6::CookiePersistentStorage::Sqlite);

    webview.load_uri(default_uri);

    // Apply settings to the webview
    apply_settings(&webview, &settings_rc.borrow());

    webview.set_vexpand(true);

    hbox.append(&top_bar);
    hbox.append(&webview);

    let webview_btn = webview.clone();
    back.connect_clicked(move |_| {
        if webview_btn.can_go_back() {
            webview_btn.go_back();
        }
    });

    let webview_btn = webview.clone();
    forward.connect_clicked(move |_| {
        if webview_btn.can_go_forward() {
            webview_btn.go_forward();
        }
    });

    let webview_btn = webview.clone();
    refresh.connect_clicked(move |_| {
        webview_btn.reload();
    });

    let notebook_btn = notebook.clone();
    let app_clone = app.clone();
    new.connect_clicked(move |_| {
        create_tab("https://start.duckduckgo.com/", &notebook_btn, &app_clone);
    });

    let app_clone = app.clone();
    let webview_btn = webview.clone();
    settings.connect_clicked(move |_| {
       create_settings_window(&app_clone, &webview_btn);
    });

    let webview_btn = webview.clone();
    search_e.connect_activate(move |entry| {
        let q = entry.text();

        let processed_q = process_search_input(&q);

        webview_btn.load_uri(&processed_q);
    });

    webview.connect_notify_local(Some("uri"), move |webview, _| {
        if let Some(uri) = webview.uri() {
            search_e.set_text(&uri);
        }
    });

    let index = notebook.append_page(&hbox, Some(&tab_box));

    let notebook_btn = notebook.clone();
    tab_close.connect_clicked(move |_| {
        notebook_btn.remove_page(Some(index));
    });

    notebook.set_current_page(Some(index));
}
