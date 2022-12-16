use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CustomDialog {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub icon_path: Option<String>,
    #[serde(default)]
    pub window_border: bool,
    #[serde(default = "default_size")]
    pub window_size: (f32, f32),
    #[serde(default = "default_pos")]
    pub window_pos: (f32, f32),
    pub body: Vec<Field>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Field {
    Label {
        text: String,
    },
    Text {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        multiline: bool,
        #[serde(default)]
        label: String,
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
    },
    Color {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default)]
        rgb: [u8; 3],
    },
    Progress {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
    },
    Check {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        #[serde(default)]
        checked: bool,
    },
    Radio {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
        selected: String,
        options: Vec<String>,
    },
    Slider {
        id: String,
        #[serde(default)]
        required: bool,
        #[serde(default)]
        label: String,
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
        options: Vec<String>,
        #[serde(default)]
        selected: String,
    },
}

fn default_date() -> NaiveDate {
    let now = Utc::now();
    NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap()
}
fn default_date_format() -> String {
    "%Y-%m-%d".to_string()
}
fn default_size() -> (f32, f32) {
    (400., 300.)
}
fn default_pos() -> (f32, f32) {
    (800., 400.)
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
