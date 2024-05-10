use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    // Path of the icon
    #[arg(long)]
    pub icon_path: Option<String>,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Notification
    Notification {
        /// Title of the notification
        #[arg(long)]
        title: String,
        /// Text of the notification
        #[arg(long)]
        text: String,
    },
    /// File/Directory selection dialog
    FileDialog {
        /// Flag for directory selection
        #[arg(long)]
        is_directory: bool,
        /// Flag for directory selection
        #[arg(long, default_value_t = String::from(""))]
        open_directory: String,
        /// Flag for multiple selection
        #[arg(long)]
        multiple: bool,
        /// Flag to be a save dialog
        #[arg(long)]
        save: bool,
    },
    /// Message dialog
    MessageDialog {
        /// Title of the window
        #[arg(long)]
        title: String,
        /// Dialog level
        #[arg(long, value_enum, default_value_t = MessageDialogLevel::Info)]
        level: MessageDialogLevel,
        /// Text of the dialog
        #[arg(long)]
        text: String,
    },
    /// Progress dialog
    Progress {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Progress"))]
        title: String,
        /// Label of the progress
        #[arg(long, default_value_t = String::from(""))]
        label: String,
    },
    /// Input with a label
    Input {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Input"))]
        title: String,
        /// Label of the input
        #[arg(long, default_value_t = String::from(""))]
        label: String,
        /// Hint text of the input
        #[arg(long)]
        hint: Option<String>,
    },
    /// Dialog to write username and password
    LogIn {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Log in"))]
        title: String,
        #[arg(long, default_value_t = String::from("Label"))]
        label: String,
        #[arg(long, default_value_t = String::from("User"))]
        user_label: String,
        #[arg(long, default_value_t = String::from("Password"))]
        pass_label: String,
    },
    /// Select a date
    Calendar {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Calendar"))]
        title: String,
        /// Label of the calendar button
        #[arg(long, default_value_t = String::from("Select date"))]
        label: String,
        /// Format of the return date (default "%Y-%m-%d")
        #[arg(long, default_value_t = String::from("%Y-%m-%d"))]
        date_format: String,
    },
    /// Select a color
    Color {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Color"))]
        title: String,
        /// Label of the color button
        #[arg(long, default_value_t = String::from("Select color"))]
        label: String,
    },
    /// Show list of strings as table
    List {
        /// Title of the window
        #[arg(long, default_value_t = String::from("List"))]
        title: String,
        /// Header of the list
        #[arg(long, default_value_t = String::from("header"))]
        header: String,
        /// Values to show in the list
        #[arg(short, long = "value")]
        values: Vec<String>,
    },
    /// Show select box with given strings
    Select {
        /// Title of the window
        #[arg(long, default_value_t = String::from("Select"))]
        title: String,
        /// Label for the select box
        #[arg(long, default_value_t = String::from("Select one"))]
        label: String,
        /// Options to select
        #[arg(short, long = "option")]
        options: Vec<String>,
    },
    /// Custom dialog
    Custom {
        /// Path of the custom dialog layout
        #[arg(long)]
        layout_path: String,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum MessageDialogLevel {
    Info,
    Warning,
    Error,
    Question,
}
