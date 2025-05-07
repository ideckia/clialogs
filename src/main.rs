use clap::Parser;
use clialogs::{
    cli::{Command, MessageDialogLevel},
    response::{Response, ResponseBody},
};
use egui::IconData;
use image::GenericImageView;
use regex::Regex;
use rfd::{FileDialog, MessageButtons, MessageDialog, MessageLevel};
use std::{collections::HashMap, fs, path::Path, vec};

fn main() {
    let cli = clialogs::cli::Cli::parse();
    let arg_icon_path = cli.icon_path;
    let (path, patterns): (String, HashMap<&str, String>) = match cli.command {
        Command::Notification { title, text } => {
            let mut not = notify_rust::Notification::new();
            not.summary(&title);
            not.body(&text);
            if let Some(icon) = arg_icon_path {
                not.icon(&icon);
            }
            match not.show() {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Error showing notification {}", err);
                }
            };
            return;
        }
        Command::FileDialog {
            is_directory,
            open_directory,
            multiple,
            save,
        } => {
            let dialog = if open_directory != "" {
                FileDialog::new().set_directory(open_directory)
            } else {
                FileDialog::new()
            };

            let opt_paths = if save {
                match dialog.save_file() {
                    Some(f) => Some(vec![f]),
                    None => None,
                }
            } else if multiple {
                if is_directory {
                    dialog.pick_folders()
                } else {
                    dialog.pick_files()
                }
            } else {
                if is_directory {
                    match dialog.pick_folder() {
                        Some(d) => Some(vec![d]),
                        None => None,
                    }
                } else {
                    match dialog.pick_file() {
                        Some(f) => Some(vec![f]),
                        None => None,
                    }
                }
            };

            if let Some(paths) = opt_paths {
                Response::ok(vec![ResponseBody {
                    id: "paths".to_string(),
                    value: format!("{:?}", paths),
                }]);
            } else {
                Response::cancel();
            }
            return;
        }
        Command::MessageDialog { title, level, text } => {
            let buttons = match level {
                MessageDialogLevel::Question => MessageButtons::OkCancel,
                _ => MessageButtons::Ok,
            };
            let ok_pressed = MessageDialog::new()
                .set_title(title.as_str())
                .set_description(text.as_str())
                .set_level(match level {
                    MessageDialogLevel::Warning => MessageLevel::Warning,
                    MessageDialogLevel::Error => MessageLevel::Error,
                    _ => MessageLevel::Info,
                })
                .set_buttons(buttons)
                .show();

            if ok_pressed {
                Response::ok(Vec::new());
            } else {
                Response::cancel();
            }
            return;
        }
        Command::Progress { title, label } => (
            "def_layouts/progress.json".to_string(),
            HashMap::from([("title", title), ("label", label)]),
        ),
        Command::Input { title, label, hint } => (
            "def_layouts/input.json".to_string(),
            HashMap::from([
                ("title", title),
                ("label", label),
                ("placeholder", hint.unwrap_or("".to_string())),
            ]),
        ),
        Command::LogIn {
            title,
            label,
            user_label,
            pass_label,
        } => (
            "def_layouts/log_in.json".to_string(),
            HashMap::from([
                ("title", title),
                ("label", label),
                ("user_label", user_label),
                ("pass_label", pass_label),
            ]),
        ),
        Command::Calendar {
            title,
            label,
            date_format,
        } => (
            "def_layouts/calendar.json".to_string(),
            HashMap::from([
                ("title", title),
                ("label", label),
                ("date-format", date_format),
            ]),
        ),
        Command::Color { title, label } => (
            "def_layouts/color.json".to_string(),
            HashMap::from([("title", title), ("label", label)]),
        ),
        Command::List {
            title,
            header,
            values,
        } => (
            "def_layouts/list.json".to_string(),
            HashMap::from([
                ("title", title),
                ("header", header),
                ("values", format!("{:?}", values)),
            ]),
        ),
        Command::Select {
            title,
            label,
            options,
        } => (
            "def_layouts/select.json".to_string(),
            HashMap::from([
                ("title", title),
                ("label", label),
                ("options", format!("{:?}", options)),
            ]),
        ),
        Command::Custom { layout_path } => (layout_path, HashMap::new()),
    };

    if path.len() == 0 {
        return;
    }

    let path_buf = std::env::current_exe().unwrap_or(std::path::PathBuf::new());
    let parent = path_buf.parent().unwrap_or(Path::new("."));
    let mut path_buf = parent.to_path_buf();
    path_buf.push(path);

    let mut data = fs::read_to_string(path_buf.as_path()).expect("Unable to read file");

    for (key, value) in patterns {
        match replace(format!("__{}__", key), value, &data.as_str()[..]) {
            Ok(d) => data = d,
            Err(err) => {
                println!("Error replacing {}", err)
            }
        }
    }

    let custom_dialog_data: clialogs::custom_dialog::CustomDialog =
        serde_json::from_str(&data).expect("Unable to parse");

    let title = custom_dialog_data.title.unwrap_or("Title".to_string());
    let mut native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_position(egui::Pos2 {
                x: custom_dialog_data.window_pos.0,
                y: custom_dialog_data.window_pos.1,
            })
            .with_decorations(!custom_dialog_data.borderless)
            .with_inner_size([
                custom_dialog_data.window_size.0,
                custom_dialog_data.window_size.1,
            ])
            .with_title(&title)
            .with_window_level(egui::WindowLevel::AlwaysOnTop),

        centered: true,
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    let icon_path = match custom_dialog_data.icon_path {
        Some(d) => Some(d),
        None => match arg_icon_path {
            Some(d) => Some(d),
            None => None,
        },
    };

    if let Some(icon) = icon_path {
        let img = image::open(icon).unwrap();
        let (width, height) = img.dimensions();

        native_options.viewport.icon = Some(std::sync::Arc::new(IconData {
            rgba: img.to_rgba8().into_raw(),
            width,
            height,
        }));
    }

    let _ = eframe::run_native(
        &title.as_str(),
        native_options,
        Box::new(|_cc| Ok(Box::new(clialogs::gui::GUI::new(custom_dialog_data.body)))),
    );
}

fn replace(target: String, replace_with: String, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target.as_str())?;
    Ok(regex.replace_all(text, replace_with.as_str()).to_string())
}
