use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

use crate::gui::{HAlign, HLabelPos, LabelPos};

#[derive(Deserialize)]
pub struct CustomDialog {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub icon_path: Option<String>,
    #[serde(default)]
    pub borderless: bool,
    #[serde(default = "default_window_size")]
    pub window_size: (f32, f32),
    #[serde(default = "default_pos")]
    pub window_pos: (f32, f32),
    pub body: Vec<Field>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Label {
        text: String,
        #[serde(default = "default_font_size")]
        font_size: f32,
    },
    Link {
        label: String,
        url: String,
        #[serde(default = "default_font_size")]
        font_size: f32,
    },
    Text {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        multiline: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        #[serde(default)]
        text: String,
        #[serde(default)]
        placeholder: String,
    },
    Calendar {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        #[serde(default = "default_date")]
        #[serde(with = "y_m_d_date_format")]
        date: NaiveDate,
        #[serde(default = "default_date_format")]
        date_format: String,
    },
    Password {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        #[serde(default)]
        text: String,
    },
    List {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        header: String,
        #[serde(default)]
        selected: String,
        values: Vec<String>,
        #[serde(default = "default_font_size")]
        font_size: f32,
    },
    Color {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        #[serde(default)]
        rgb: [u8; 3],
    },
    Progress {
        id: String,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
    },
    Check {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_h_label_pos")]
        label_pos: HLabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        #[serde(default)]
        checked: bool,
    },
    Radio {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        selected: String,
        options: Vec<String>,
    },
    Slider {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_label_pos")]
        label_pos: LabelPos,
        #[serde(default = "default_font_size")]
        font_size: f32,
        min: f32,
        max: f32,
        value: f32,
        #[serde(default)]
        suffix: String,
    },
    Combobox {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default = "default_font_size")]
        font_size: f32,
        options: Vec<String>,
        #[serde(default)]
        selected: String,
    },
    Image {
        id: String,
        #[serde(default)]
        path: String,
        #[serde(default = "default_image_scale")]
        scale: f32,
        #[serde(default = "default_h_align")]
        h_align: HAlign,
    },
}

fn default_date() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap()
}
fn default_date_format() -> String {
    "%Y-%m-%d".to_string()
}
fn default_window_size() -> (f32, f32) {
    (400., 300.)
}
fn default_font_size() -> f32 {
    16.0
}
fn default_pos() -> (f32, f32) {
    (800., 400.)
}
fn default_label_pos() -> LabelPos {
    LabelPos::Over
}
fn default_h_label_pos() -> HLabelPos {
    HLabelPos::After
}
fn default_image_scale() -> f32 {
    1.0
}
fn default_h_align() -> HAlign {
    HAlign::Center
}

mod y_m_d_date_format {

    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match chrono::NaiveDate::parse_from_str(&s, FORMAT) {
            Ok(naive_date) => Ok(naive_date),
            Err(msg) => Err(serde::de::Error::custom(msg)),
        }
    }
}
