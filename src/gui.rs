use egui::Ui;
use egui::Vec2;
use egui::Widget;
use egui_extras::{Column, TableBuilder};
use mpsc::{Receiver, Sender};
use serde::Deserialize;
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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelPos {
    Over,
    Next,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HLabelPos {
    Before,
    After,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HAlign {
    Left,
    Center,
    Right,
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
        label_pos: &LabelPos,
        font_size: f32,
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

        match label_pos {
            LabelPos::Over => {
                ui.vertical(|ui| {
                    let label_w = if mark_as_required {
                        egui::RichText::new(label_text.as_str())
                            .size(font_size)
                            .color(egui::Color32::RED)
                    } else {
                        egui::RichText::new(label_text.as_str()).size(font_size)
                    };
                    ui.label(label_w);
                    ui.add(widget);
                });
            }
            LabelPos::Next => {
                ui.horizontal(|ui| {
                    let label_w = if mark_as_required {
                        egui::RichText::new(label_text.as_str())
                            .size(font_size)
                            .color(egui::Color32::RED)
                    } else {
                        egui::RichText::new(label_text.as_str()).size(font_size)
                    };
                    ui.label(label_w);
                    ui.add(widget);
                });
            }
        }
    }

    fn handle_user_input(tx: Sender<UserInput>) {
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let mut user_input = String::new();
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let window_rect = ctx.input(|i: &egui::InputState| i.screen_rect());
        let window_size = Vec2::new(
            window_rect.max.x - window_rect.min.x,
            window_rect.max.y - window_rect.min.y,
        );
        let bottom_line_height = 60.;
        let is_shift = ctx.input(|i| i.modifiers.shift);
        let is_enter = ctx.input(|i| i.key_released(egui::Key::Enter));

        if is_enter && !is_shift {
            self.ok_pressed = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let is_escape = ctx.input(|i| i.key_released(egui::Key::Escape));
        if is_escape {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
                            Field::Label { text, font_size } => {
                                ui.label(egui::RichText::new(text.as_str()).size(*font_size));
                            }
                            Field::Link {
                                label,
                                url,
                                font_size,
                            } => {
                                ui.hyperlink_to(
                                    egui::RichText::new(label.as_str()).size(*font_size),
                                    url,
                                );
                            }
                            Field::Text {
                                id: _,
                                required,
                                multiline,
                                label,
                                label_pos,
                                font_size,
                                text,
                                placeholder,
                            } => {
                                let mark_as_required = *required && text.len() == 0;
                                let text_edit = if *multiline {
                                    egui::TextEdit::multiline(text)
                                        .desired_width(window_size.x)
                                        .font(egui::FontId::new(
                                            *font_size,
                                            egui::FontFamily::Proportional,
                                        ))
                                } else {
                                    egui::TextEdit::singleline(text)
                                        .desired_width(window_size.x)
                                        .font(egui::FontId::new(
                                            *font_size,
                                            egui::FontFamily::Proportional,
                                        ))
                                };
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    &label_pos,
                                    *font_size,
                                    text_edit.hint_text(placeholder.as_str()),
                                    mark_as_required,
                                );
                            }
                            Field::Calendar {
                                id: _,
                                required: _,
                                label,
                                label_pos,
                                font_size,
                                date,
                                date_format: _,
                            } => {
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    &label_pos,
                                    *font_size,
                                    egui_extras::DatePickerButton::new(date),
                                    false,
                                );
                            }
                            Field::Password {
                                id: _,
                                required,
                                label,
                                label_pos,
                                font_size,
                                text,
                            } => {
                                let mark_as_required = *required && text.len() == 0;
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    &label_pos,
                                    *font_size,
                                    egui::TextEdit::singleline(text)
                                        .password(true)
                                        .desired_width(window_size.x)
                                        .font(egui::FontId::new(
                                            *font_size,
                                            egui::FontFamily::Proportional,
                                        )),
                                    mark_as_required,
                                );
                            }
                            Field::List {
                                id: _,
                                required,
                                header,
                                selected,
                                values,
                                font_size,
                            } => {
                                TableBuilder::new(ui)
                                    .striped(true)
                                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                                    .column(Column::remainder().at_least(100.0))
                                    .header(20.0, |mut row| {
                                        row.col(|ui| {
                                            let heading = if *required && selected.len() == 0 {
                                                egui::RichText::new(header.as_str())
                                                    .size(*font_size)
                                                    .color(egui::Color32::RED)
                                            } else {
                                                egui::RichText::new(header.as_str())
                                                    .size(*font_size)
                                            };
                                            ui.heading(heading);
                                        });
                                    })
                                    .body(|mut body| {
                                        for v in values.iter() {
                                            body.row(18.0, |mut row| {
                                                row.col(|ui| {
                                                    ui.selectable_value(
                                                        selected,
                                                        v.to_string(),
                                                        egui::RichText::new(v.as_str())
                                                            .size(*font_size),
                                                    );
                                                });
                                            });
                                        }
                                    });
                            }
                            Field::Color {
                                id: _,
                                required: _,
                                label,
                                label_pos,
                                font_size,
                                rgb,
                            } => match label_pos {
                                LabelPos::Over => {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new(label.as_str()).size(*font_size),
                                        );
                                        egui::widgets::color_picker::color_edit_button_srgb(
                                            ui, rgb,
                                        );
                                    });
                                }
                                LabelPos::Next => {
                                    ui.horizontal(|ui| {
                                        ui.label(label.as_str());
                                        egui::widgets::color_picker::color_edit_button_srgb(
                                            ui, rgb,
                                        );
                                    });
                                }
                            },
                            Field::Progress {
                                id: _,
                                label,
                                label_pos,
                                font_size,
                            } => {
                                match label_pos {
                                    LabelPos::Over => {
                                        ui.vertical(|ui| {
                                            ui.label(
                                                egui::RichText::new(label.as_str())
                                                    .size(*font_size),
                                            );
                                            ui.add(
                                                egui::ProgressBar::new(self.progress / 100.)
                                                    .show_percentage()
                                                    .desired_width(window_size.x),
                                            );
                                            // ui.add(egui::Spinner::new());
                                        });
                                    }
                                    LabelPos::Next => {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                egui::RichText::new(label.as_str())
                                                    .size(*font_size),
                                            );
                                            ui.add(
                                                egui::ProgressBar::new(self.progress / 100.)
                                                    .show_percentage()
                                                    .desired_width(window_size.x),
                                            );
                                            ui.add(egui::Spinner::new());
                                        });
                                    }
                                }
                                ctx.request_repaint();
                            }
                            Field::Check {
                                id: _,
                                required,
                                label,
                                label_pos,
                                font_size,
                                checked,
                            } => match label_pos {
                                HLabelPos::Before => {
                                    ui.horizontal(|ui| {
                                        let label_w = if *required && !*checked {
                                            egui::RichText::new(label.as_str())
                                                .size(*font_size)
                                                .color(egui::Color32::RED)
                                        } else {
                                            egui::RichText::new(label.as_str()).size(*font_size)
                                        };
                                        ui.label(label_w);
                                        ui.checkbox(checked, "");
                                    });
                                }
                                HLabelPos::After => {
                                    let label_w = if *required && !*checked {
                                        egui::RichText::new(label.as_str())
                                            .size(*font_size)
                                            .color(egui::Color32::RED)
                                    } else {
                                        egui::RichText::new(label.as_str()).size(*font_size)
                                    };
                                    ui.checkbox(checked, label_w);
                                }
                            },
                            Field::Radio {
                                id: _,
                                required,
                                label,
                                label_pos,
                                font_size,
                                selected,
                                options,
                            } => match label_pos {
                                LabelPos::Over => {
                                    ui.vertical(|ui| {
                                        let label_w = if *required && selected.len() == 0 {
                                            egui::RichText::new(label.as_str())
                                                .size(*font_size)
                                                .color(egui::Color32::RED)
                                        } else {
                                            egui::RichText::new(label.as_str()).size(*font_size)
                                        };
                                        ui.label(label_w);
                                        for opt in options.iter() {
                                            ui.radio_value(
                                                selected,
                                                opt.to_string(),
                                                egui::RichText::new(opt.as_str()).size(*font_size),
                                            );
                                        }
                                    });
                                }
                                LabelPos::Next => {
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
                                            ui.radio_value(
                                                selected,
                                                opt.to_string(),
                                                egui::RichText::new(opt.as_str()).size(*font_size),
                                            );
                                        }
                                    });
                                }
                            },
                            Field::Slider {
                                id: _,
                                required: _,
                                label,
                                label_pos,
                                font_size,
                                min,
                                max,
                                value,
                                suffix,
                            } => {
                                GUI::add_labeled_widget(
                                    ui,
                                    &label,
                                    &label_pos,
                                    *font_size,
                                    egui::Slider::new(value, *min..=*max).suffix(suffix.as_str()),
                                    false,
                                );
                            }
                            Field::Combobox {
                                id: _,
                                required,
                                label,
                                font_size,
                                options,
                                selected,
                            } => {
                                let label_w = if *required && selected.len() == 0 {
                                    egui::RichText::new(label.as_str())
                                        .size(*font_size)
                                        .color(egui::Color32::RED)
                                } else {
                                    egui::RichText::new(label.as_str()).size(*font_size)
                                };
                                let combo = egui::ComboBox::from_label(label_w);
                                combo.selected_text(selected.as_str()).show_ui(ui, |ui| {
                                    for opt in options {
                                        ui.selectable_value(
                                            selected,
                                            opt.to_string(),
                                            egui::RichText::new(opt.as_str()).size(*font_size),
                                        );
                                    }
                                });
                            }
                            Field::Image {
                                id: _,
                                path,
                                scale,
                                h_align,
                            } => {
                                ui.horizontal(|ui| {
                                    let img = egui::Image::new(path.to_string())
                                        .fit_to_original_size(*scale);
                                    let tpoll =
                                        img.load_for_size(ui.ctx(), ui.available_size()).unwrap();
                                    let mut img_w = 0.;

                                    match tpoll.size() {
                                        Some(s) => {
                                            img_w = s.x * *scale;
                                        }
                                        None => {}
                                    }
                                    let space = match h_align {
                                        HAlign::Left => 0.,
                                        HAlign::Center => {
                                            (window_size.x - ui.spacing().item_spacing.x - img_w)
                                                * 0.5
                                        }
                                        HAlign::Right => {
                                            window_size.x - ui.spacing().item_spacing.x - img_w
                                        }
                                    };

                                    ui.add_space(space);
                                    ui.add(img);
                                });
                            }
                        });
                    }
                });
            ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                ui.set_max_size(Vec2::new(window_size.x, 30.0));
                ui.vertical(|ui| {
                    ui.separator();
                    ui.horizontal_centered(|ui| {
                        let buttons_width = 80.;
                        ui.add_space((window_size.x - buttons_width) * 0.5);
                        if ui.button("Ok").clicked() {
                            self.ok_pressed = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        if ui.button("Cancel").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
            });
        });

        let close_requested = ctx.input(|i| i.viewport().close_requested());

        if close_requested && !confirm_close(self) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
    }

    fn on_exit(&mut self) {}

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

fn confirm_close(gui: &mut GUI) -> bool {
    if !gui.ok_pressed {
        Response::cancel();
        return true;
    }

    let mut close_window = true;
    let out: Vec<ResponseBody> = gui
        .custom_dialog_fields
        .iter()
        .filter_map(|field| match field {
            Field::Label {
                text: _,
                font_size: _,
            } => None,
            Field::Link {
                label: _,
                url: _,
                font_size: _,
            } => None,
            Field::Text {
                id,
                required,
                multiline: _,
                label: _,
                label_pos: _,
                font_size: _,
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
                label_pos: _,
                font_size: _,
                date,
                date_format,
            } => Some(ResponseBody {
                id: id.to_string(),
                value: format!("{}", date.format(&date_format)),
            }),
            Field::Password {
                id,
                required,
                label: _,
                label_pos: _,
                font_size: _,
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
                font_size: _,
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
                label_pos: _,
                font_size: _,
                rgb,
            } => Some(ResponseBody {
                id: id.to_string(),
                value: format!("[{},{},{}]", rgb[0], rgb[1], rgb[2]),
            }),
            Field::Progress {
                id: _,
                label: _,
                label_pos: _,
                font_size: _,
            } => None,
            Field::Check {
                id,
                required,
                label: _,
                label_pos: _,
                font_size: _,
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
                label_pos: _,
                font_size: _,
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
                label_pos: _,
                font_size: _,
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
                font_size: _,
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
            Field::Image {
                id,
                path,
                scale: _,
                h_align: _,
            } => Some(ResponseBody {
                id: id.to_string(),
                value: path.to_string(),
            }),
        })
        .collect();
    if close_window {
        Response::ok(out);
    }

    gui.ok_pressed = false;

    close_window
}
