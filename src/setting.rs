use gtk4::glib::Propagation;
use gtk4::{
    prelude::*, ApplicationWindow, Box, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, Stack, Switch
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use webkit6::WebView;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebkitSettings {
    categories: Vec<CategorySettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CategorySettings {
    name: String,
    settings: Vec<Setting>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Setting {
    key: String,
    value: String,
}

const SETTINGS_FILE: &str = "settings.json";

pub fn load_settings() -> Rc<RefCell<WebkitSettings>> {
    if Path::new(SETTINGS_FILE).exists() {
        let data = fs::read_to_string(SETTINGS_FILE).expect("Unable to read settings file");
        let settings: WebkitSettings = serde_json::from_str(&data).expect("Unable to parse JSON");
        Rc::new(RefCell::new(settings))
    } else {
        let default_settings = WebkitSettings {
            categories: vec![
                CategorySettings {
                    name: "General Settings".to_string(),
                    settings: vec![
                        Setting {
                            key: "Enable JavaScript".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Zoom Text Only".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Print Backgrounds".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Auto Load Images".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Allow Modal Dialogs".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Allow File Access from File URLs".to_string(),
                            value: "false".to_string(),
                        },
                    ],
                },
                CategorySettings {
                    name: "Media Settings".to_string(),
                    settings: vec![
                        Setting {
                            key: "Media Playback Requires User Gesture".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Media Playback Allows Inline".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Media".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable WebRTC".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Media Stream".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Media Capabilities".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Encrypted Media".to_string(),
                            value: "true".to_string(),
                        },
                    ],
                },
                CategorySettings {
                    name: "JavaScript Settings".to_string(),
                    settings: vec![
                        Setting {
                            key: "JavaScript Can Open Windows Automatically".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "JavaScript Can Access Clipboard".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable JavaScript Markup".to_string(),
                            value: "false".to_string(),
                        },
                    ],
                },
                CategorySettings {
                    name: "Web Features".to_string(),
                    settings: vec![
                        Setting {
                            key: "Enable Tabs to Links".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Spatial Navigation".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Smooth Scrolling".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Resizable Text Areas".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Page Cache".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable Offline Web Application Cache".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable HTML5 Local Storage".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable HTML5 Database".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Fullscreen".to_string(),
                            value: "true".to_string(),
                        },
                        Setting {
                            key: "Enable DNS Prefetching".to_string(),
                            value: "true".to_string(),
                        },
                    ],
                },
                CategorySettings {
                    name: "Security Settings".to_string(),
                    settings: vec![
                        Setting {
                            key: "Disable Web Security".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Allow Universal Access from File URLs".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Allow Top Navigation to Data URLs".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Developer Extras".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Hyperlink Auditing".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Draw Compositing Indicators".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Mock Capture Devices".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Site-Specific Quirks".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Back Forward Navigation Gestures".to_string(),
                            value: "false".to_string(),
                        },
                        Setting {
                            key: "Enable Write Console Messages to Stdout".to_string(),
                            value: "false".to_string(),
                        },
                    ],
                },
            ],
        };
        save_settings(&default_settings);
        Rc::new(RefCell::new(default_settings))
    }
}


fn save_settings(settings: &WebkitSettings) {
    let json = serde_json::to_string_pretty(&settings).expect("Failed to serialize settings");
    fs::write(SETTINGS_FILE, json).expect("Unable to write to settings file");
}

pub fn apply_settings(webview: &WebView, settings: &WebkitSettings) {
    if let Some(web_settings) = webkit6::prelude::WebViewExt::settings(webview) {
        for category in &settings.categories {
            for setting in &category.settings {
                match setting.key.as_str() {
                    "Enable JavaScript" => {
                        let enable_js = setting.value == "true";
                        web_settings.set_enable_javascript(enable_js);
                    },
                    "Zoom Text Only" => {
                        let enable_zoom_text_only = setting.value == "true";
                        web_settings.set_zoom_text_only(enable_zoom_text_only);
                    },
                    "Print Backgrounds" => {
                        let print_backgrounds = setting.value == "true";
                        web_settings.set_print_backgrounds(print_backgrounds);
                    },
                    "Media Playback Requires User Gesture" => {
                        let requires_user_gesture = setting.value == "true";
                        web_settings.set_media_playback_requires_user_gesture(requires_user_gesture);
                    },
                    "Media Playback Allows Inline" => {
                        let allows_inline = setting.value == "true";
                        web_settings.set_media_playback_allows_inline(allows_inline);
                    },
                    "Load Icons Ignoring Image Load Setting" => {
                        let load_icons = setting.value == "true";
                        web_settings.set_load_icons_ignoring_image_load_setting(load_icons);
                    },
                    "JavaScript Can Open Windows Automatically" => {
                        let can_open_windows = setting.value == "true";
                        web_settings.set_javascript_can_open_windows_automatically(can_open_windows);
                    },
                    "JavaScript Can Access Clipboard" => {
                        let can_access_clipboard = setting.value == "true";
                        web_settings.set_javascript_can_access_clipboard(can_access_clipboard);
                    },
                    "Enable Write Console Messages to Stdout" => {
                        let enable_write_console = setting.value == "true";
                        web_settings.set_enable_write_console_messages_to_stdout(enable_write_console);
                    },
                    "Enable WebRTC" => {
                        let enable_webrtc = setting.value == "true";
                        web_settings.set_enable_webrtc(enable_webrtc);
                    },
                    "Enable WebGL" => {
                        let enable_webgl = setting.value == "true";
                        web_settings.set_enable_webgl(enable_webgl);
                    },
                    "Enable WebAudio" => {
                        let enable_webaudio = setting.value == "true";
                        web_settings.set_enable_webaudio(enable_webaudio);
                    },
                    "Enable Tabs to Links" => {
                        let enable_tabs = setting.value == "true";
                        web_settings.set_enable_tabs_to_links(enable_tabs);
                    },
                    "Enable Spatial Navigation" => {
                        let enable_spatial_navigation = setting.value == "true";
                        web_settings.set_enable_spatial_navigation(enable_spatial_navigation);
                    },
                    "Enable Smooth Scrolling" => {
                        let enable_smooth_scrolling = setting.value == "true";
                        web_settings.set_enable_smooth_scrolling(enable_smooth_scrolling);
                    },
                    "Enable Site-Specific Quirks" => {
                        let enable_site_specific_quirks = setting.value == "true";
                        web_settings.set_enable_site_specific_quirks(enable_site_specific_quirks);
                    },
                    "Enable Resizable Text Areas" => {
                        let enable_resizable_text_areas = setting.value == "true";
                        web_settings.set_enable_resizable_text_areas(enable_resizable_text_areas);
                    },
                    "Enable Page Cache" => {
                        let enable_page_cache = setting.value == "true";
                        web_settings.set_enable_page_cache(enable_page_cache);
                    },
                    "Enable Offline Web Application Cache" => {
                        let enable_offline_cache = setting.value == "true";
                        web_settings.set_enable_offline_web_application_cache(enable_offline_cache);
                    },
                    "Enable Mock Capture Devices" => {
                        let enable_mock_capture = setting.value == "true";
                        web_settings.set_enable_mock_capture_devices(enable_mock_capture);
                    },
                    "Enable Media Stream" => {
                        let enable_media_stream = setting.value == "true";
                        web_settings.set_enable_media_stream(enable_media_stream);
                    },
                    "Enable Media Capabilities" => {
                        let enable_media_capabilities = setting.value == "true";
                        web_settings.set_enable_media_capabilities(enable_media_capabilities);
                    },
                    "Enable Media" => {
                        let enable_media = setting.value == "true";
                        web_settings.set_enable_media(enable_media);
                    },
                    "Enable JavaScript Markup" => {
                        let enable_js_markup = setting.value == "true";
                        web_settings.set_enable_javascript_markup(enable_js_markup);
                    },
                    "Enable Hyperlink Auditing" => {
                        let enable_hyperlink_auditing = setting.value == "true";
                        web_settings.set_enable_hyperlink_auditing(enable_hyperlink_auditing);
                    },
                    "Enable HTML5 Local Storage" => {
                        let enable_local_storage = setting.value == "true";
                        web_settings.set_enable_html5_local_storage(enable_local_storage);
                    },
                    "Enable HTML5 Database" => {
                        let enable_database = setting.value == "true";
                        web_settings.set_enable_html5_database(enable_database);
                    },
                    "Enable Fullscreen" => {
                        let enable_fullscreen = setting.value == "true";
                        web_settings.set_enable_fullscreen(enable_fullscreen);
                    },
                    "Enable Encrypted Media" => {
                        let enable_encrypted_media = setting.value == "true";
                        web_settings.set_enable_encrypted_media(enable_encrypted_media);
                    },
                    "Enable DNS Prefetching" => {
                        let enable_dns_prefetching = setting.value == "true";
                        web_settings.set_enable_dns_prefetching(enable_dns_prefetching);
                    },
                    "Enable Developer Extras" => {
                        let enable_developer_extras = setting.value == "true";
                        web_settings.set_enable_developer_extras(enable_developer_extras);
                    },
                    "Enable Caret Browsing" => {
                        let enable_caret_browsing = setting.value == "true";
                        web_settings.set_enable_caret_browsing(enable_caret_browsing);
                    },
                    "Enable Back Forward Navigation Gestures" => {
                        let enable_back_forward_nav = setting.value == "true";
                        web_settings.set_enable_back_forward_navigation_gestures(enable_back_forward_nav);
                    },
                    "Draw Compositing Indicators" => {
                        let draw_compositing_indicators = setting.value == "true";
                        web_settings.set_draw_compositing_indicators(draw_compositing_indicators);
                    },
                    "Disable Web Security" => {
                        let disable_web_security = setting.value == "true";
                        web_settings.set_disable_web_security(disable_web_security);
                    },
                    "Auto Load Images" => {
                        let auto_load_images = setting.value == "true";
                        web_settings.set_auto_load_images(auto_load_images);
                    },
                    "Allow Universal Access from File URLs" => {
                        let allow_universal_access = setting.value == "true";
                        web_settings.set_allow_universal_access_from_file_urls(allow_universal_access);
                    },
                    "Allow Top Navigation to Data URLs" => {
                        let allow_top_navigation = setting.value == "true";
                        web_settings.set_allow_top_navigation_to_data_urls(allow_top_navigation);
                    },
                    "Allow Modal Dialogs" => {
                        let allow_modal_dialogs = setting.value == "true";
                        web_settings.set_allow_modal_dialogs(allow_modal_dialogs);
                    },
                    "Allow File Access from File URLs" => {
                        let allow_file_access = setting.value == "true";
                        web_settings.set_allow_file_access_from_file_urls(allow_file_access);
                    },
                    _ => println!("Unknown setting: {}", setting.key),
                }
            }
        }
    }
}

pub fn create_settings_window(application: &gtk4::Application, webview: &WebView) {
    let window = ApplicationWindow::new(application);
    window.set_title(Some("aapelix/rubra/settings"));
    window.set_default_size(900, 600);

    let settings = load_settings();

    let vbox = Box::new(Orientation::Horizontal, 10);
    let stack = Stack::new();
    let sidebar = ListBox::new();
    sidebar.set_vexpand(true);

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_vexpand(true);
    scrolled_window.set_hexpand(true);

    for category in settings.borrow_mut().categories.iter_mut() {
        // Create a button for each category in the sidebar
        let button = Button::with_label(&category.name);
        let category_clone = category.name.clone();
        let stack_clone = stack.clone();

        button.connect_clicked(move |_| {
            stack_clone.set_visible_child_name(&category_clone);
        });

        sidebar.append(&button);
    }

    // Create a scrolled window for the settings
    for category in settings.borrow_mut().categories.iter_mut() {
        let category_box = ListBox::new();
        for setting in category.settings.iter_mut() {
            let row = ListBoxRow::new();

            let hbox = Box::new(Orientation::Horizontal, 10);
            let label = Label::new(Some(&setting.key));
            hbox.append(&label);

            let toggle_switch = Switch::new();
            toggle_switch.set_active(setting.value == "true");
            hbox.append(&toggle_switch);

            let setting_key = setting.key.clone();
            let settings_clone = Rc::clone(&settings);
            let webview_clone = webview.clone();

            toggle_switch.connect_state_set(move |_, state| {
                let new_value = if state { "true".to_string() } else { "false".to_string() };
                println!("Setting '{}' changed to {}", setting_key, new_value);

                settings_clone.borrow_mut().categories.iter_mut()
                    .flat_map(|c| c.settings.iter_mut())
                    .find(|s| s.key == setting_key)
                    .map(|s| s.value = new_value);

                save_settings(&settings_clone.borrow());

                apply_settings(&webview_clone, &settings_clone.borrow());

                Propagation::Stop
            });

            row.set_child(Some(&hbox));
            category_box.append(&row);
        }
        stack.add_titled(&category_box, Some(&category.name), &category.name);
    }

    vbox.append(&sidebar); // Add sidebar to the main vertical box
    vbox.append(&scrolled_window); // Add scrolled window for the settings
    scrolled_window.set_child(Some(&stack)); // Set the stack in the scrolled window

    window.set_child(Some(&vbox)); // Set the main vertical box in the window
    window.show();

    apply_settings(webview, &settings.borrow());
}
