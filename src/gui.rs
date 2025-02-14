use egui::Ui;
use egui::Widget;
use egui_extras::{Column, TableBuilder};
use mpsc::{Receiver, Sender};
use std::sync::mpsc;

use crate::custom_dialog::Field;
use crate::response::Response;
use crate::response::ResponseBody;

pub struct GUI {
    custom_dialog_fields: Vec<Field>,
    ok_pressed: bool,
    rx: Receiver<UserInput>,
    progress: f32,
}
enum UserInput {
    Progress(f32),
}

impl GUI {
    pub fn new(custom_dialog_fields: Vec<Field>) -> Self {
        let (tx, rx): (Sender<UserInput>, Receiver<UserInput>) = mpsc::channel();
        Self::handle_user_input(tx);
        GUI {
            custom_dialog_fields,
            ok_pressed: false,
            rx,
            progress: 0f32,
        }
    }

    pub fn add_labeled_widget(
        ui: &mut Ui,
        label_text: &String,
        widget: impl Widget,
        mark_as_required: bool,
    ) {
        if label_text.len() == 0 || label_text == "null" {
            if mark_as_required {
                ui.visuals_mut().extreme_bg_color = egui::Color32::LIGHT_RED;
            }
            ui.add(widget);
            return;
        }

        ui.vertical(|ui| {
            if mark_as_required {
                ui.label(egui::RichText::new(label_text.as_str()).color(egui::Color32::RED));
            } else {
                ui.label(label_text.as_str());
            }
            ui.add(widget);
        });
    }

    fn handle_user_input(tx: Sender<UserInput>) {
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let mut user_input = String::new();
            // let stop_string = String::from("progress");
            let progress_regex = regex::Regex::new("progress-([0-9]+)").unwrap();
            let sleep_duration = std::time::Duration::from_millis(100);

            loop {
                std::thread::sleep(sleep_duration);
                user_input.clear();
                match stdin.read_line(&mut user_input) {
                    Ok(_) => {}
                    Err(error) => println!("error reading user input: {error}"),
                }

                user_input.retain(|c| c.is_ascii());

                match progress_regex.captures(user_input.as_str()) {
                    Some(captured_progress) => {
                        let p = captured_progress.get(1).unwrap().as_str();
                        tx.send(UserInput::Progress(p.parse().unwrap())).unwrap();
                    }
                    None => {
                        println!("not captured -> {:?}", user_input)
                    }
                }
            }
        });
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let window_size = frame.info().window_info.size;
        let bottom_line_height = 60.;
        if ctx.input().key_released(egui::Key::Enter) && !ctx.input().modifiers.shift {
            self.ok_pressed = true;
            frame.close();
        }
        if ctx.input().key_released(egui::Key::Escape) {
            frame.close();
        }
        match self.rx.try_recv() {
            Ok(user_input) => match user_input {
                UserInput::Progress(progress) => {
                    self.progress = progress;
                }
            },
            Err(_) => {}
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);
            egui::ScrollArea::vertical()
                .max_height(window_size.y - bottom_line_height)
                .show(ui, |ui| {
                    for (index, field) in self.custom_dialog_fields.iter_mut().enumerate() {
                        ui.push_id(index, |ui| match field {
                            Field::Label { text } => {
                                ui.label(text.as_str());
                            }
                            Field::Link { label, url } => {
                                ui.hyperlink_to(label.as_str(), url);
                            }
                            Field::Text {
                                id: _,
                                required,
                                multiline,
                                label,
                                text,
                                placeholder,
                            } => {
                                let mark_as_required = *required && text.len() == 0;
                                let text_edit = if *multiline {
                                    egui::TextEdit::multiline(text).desired_width(window_size.x)
                                } else {
                                    egui::TextEdit::singleline(text).desired_width(window_size.x)
                                };
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    text_edit.hint_text(placeholder.as_str()),
                                    mark_as_required,
                                );
                            }
                            Field::Calendar {
                                id: _,
                                required: _,
                                label,
                                date,
                                date_format: _,
                            } => {
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    egui_extras::DatePickerButton::new(date),
                                    false,
                                );
                            }
                            Field::Password {
                                id: _,
                                required,
                                label,
                                text,
                            } => {
                                let mark_as_required = *required && text.len() == 0;
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    egui::TextEdit::singleline(text)
                                        .password(true)
                                        .desired_width(window_size.x),
                                    mark_as_required,
                                );
                            }
                            Field::List {
                                id: _,
                                required,
                                header,
                                selected,
                                values,
                            } => {
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::remainder().at_least(100.0))
                                    .header(20.0, |mut row| {
                                        row.col(|ui| {
                                            if *required && selected.len() == 0 {
                                                ui.heading(
                                                    egui::RichText::new(header.as_str())
                                                        .color(egui::Color32::RED),
                                                );
                                            } else {
                                                ui.heading(header);
                                            }
                                        });
                                    })
                                    .body(|mut body| {
                                        for v in values.iter() {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.selectable_value(selected, v.to_string(), v);
                                                });
                                            });
                                        }
                                    });
                            }
                            Field::Color {
                                id: _,
                                required: _,
                                label,
                                rgb,
                            } => {
                                ui.horizontal(|ui| {
                                    ui.label(label.as_str());
                                    egui::widgets::color_picker::color_edit_button_srgb(ui, rgb);
                                });
                            }
                            Field::Progress {
                                id: _,
                                required: _,
                                label,
                            } => {
                                ui.horizontal(|ui| {
                                    ui.label(label.as_str());
                                    ui.add(
                                        egui::ProgressBar::new(self.progress / 100.)
                                            .show_percentage()
                                            .desired_width(window_size.x),
                                    );
                                    ui.add(egui::Spinner::new());
                                });
                                ctx.request_repaint();
                            }
                            Field::Check {
                                id: _,
                                required,
                                label,
                                checked,
                            } => {
                                if *required && !*checked {
                                    ui.checkbox(
                                        checked,
                                        egui::RichText::new(label.as_str())
                                            .color(egui::Color32::RED),
                                    );
                                } else {
                                    ui.checkbox(checked, label.as_str());
                                }
                            }
                            Field::Radio {
                                id: _,
                                required,
                                label,
                                selected,
                                options,
                            } => {
                                ui.horizontal(|ui| {
                                    if *required && selected.len() == 0 {
                                        ui.label(
                                            egui::RichText::new(label.as_str())
                                                .color(egui::Color32::RED),
                                        );
                                    } else {
                                        ui.label(label.as_str());
                                    }
                                    for opt in options.iter() {
                                        ui.radio_value(selected, opt.to_string(), opt);
                                    }
                                });
                            }
                            Field::Slider {
                                id: _,
                                required: _,
                                label,
                                min,
                                max,
                                value,
                                suffix,
                            } => {
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    egui::Slider::new(value, *min..=*max).suffix(suffix.as_str()),
                                    false,
                                );
                            }
                            Field::Combobox {
                                id: _,
                                required,
                                label,
                                options,
                                selected,
                            } => {
                                let combo = if *required && selected.len() == 0 {
                                    egui::ComboBox::from_label(
                                        egui::RichText::new(label.as_str())
                                            .color(egui::Color32::RED),
                                    )
                                } else {
                                    egui::ComboBox::from_label(egui::RichText::new(label.as_str()))
                                };
                                combo.selected_text(selected.as_str()).show_ui(ui, |ui| {
                                    for opt in options {
                                        ui.selectable_value(
                                            selected,
                                            opt.to_string(),
                                            opt.as_str(),
                                        );
                                    }
                                });
                            }
                        });
                    }
                });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                ui.horizontal(|ui| {
                    let buttons_width = 80.;
                    ui.add_space((window_size.x - buttons_width * 1.5) * 0.5);
                    if ui.button("Cancel").clicked() {
                        frame.close();
                    }
                    if ui.button("Ok").clicked() {
                        self.ok_pressed = true;
                        frame.close();
                    }
                });
                ui.separator();
            });
        });
    }

    fn on_close_event(&mut self) -> bool {
        if !self.ok_pressed {
            Response::cancel();
            return true;
        }

        let mut close_window = true;
        let out: Vec<ResponseBody> = self
            .custom_dialog_fields
            .iter()
            .filter_map(|field| match field {
                Field::Label { text: _ } => None,
                Field::Link { label: _, url: _ } => None,
                Field::Text {
                    id,
                    required,
                    multiline: _,
                    label: _,
                    text,
                    placeholder: _,
                } => {
                    if *required && text.len() == 0 {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: text.to_string(),
                    })
                }
                Field::Calendar {
                    id,
                    required: _,
                    label: _,
                    date,
                    date_format,
                } => Some(ResponseBody {
                    id: id.to_string(),
                    value: format!("{}", date.format(date_format)),
                }),
                Field::Password {
                    id,
                    required,
                    label: _,
                    text,
                } => {
                    if *required && text.len() == 0 {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: text.to_string(),
                    })
                }
                Field::List {
                    id,
                    required,
                    header: _,
                    selected,
                    values: _,
                } => {
                    if *required && selected.len() == 0 {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: selected.to_string(),
                    })
                }
                Field::Color {
                    id,
                    required: _,
                    label: _,
                    rgb,
                } => Some(ResponseBody {
                    id: id.to_string(),
                    value: format!("[{},{},{}]", rgb[0], rgb[1], rgb[2]),
                }),
                Field::Progress {
                    id: _,
                    required: _,
                    label: _,
                } => None,
                Field::Check {
                    id,
                    required,
                    label: _,
                    checked,
                } => {
                    if *required && !checked {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: checked.to_string(),
                    })
                }
                Field::Radio {
                    id,
                    required,
                    label: _,
                    selected,
                    options: _,
                } => {
                    if *required && selected.len() == 0 {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: selected.to_string(),
                    })
                }
                Field::Slider {
                    id,
                    required: _,
                    label: _,
                    min: _,
                    max: _,
                    value,
                    suffix: _,
                } => Some(ResponseBody {
                    id: id.to_string(),
                    value: value.to_string(),
                }),
                Field::Combobox {
                    id,
                    required,
                    label: _,
                    options: _,
                    selected,
                } => {
                    if *required && selected.len() == 0 {
                        close_window = false;
                    }
                    Some(ResponseBody {
                        id: id.to_string(),
                        value: selected.to_string(),
                    })
                }
            })
            .collect();
        if close_window {
            Response::ok(out);
        }

        self.ok_pressed = false;

        close_window
    }

    fn on_exit(&mut self) {}

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
