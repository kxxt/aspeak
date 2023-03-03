use std::fmt::Display;

use colored::Colorize;
use serde::Deserialize;

/// Voice information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Voice {
    pub display_name: String,
    pub gender: String,
    pub local_name: String,
    pub locale: String,
    pub locale_name: String,
    pub name: String,
    pub sample_rate_hertz: String,
    pub short_name: String,
    pub status: String,
    pub voice_type: String,
    pub words_per_minute: Option<String>,
    pub style_list: Option<Vec<String>>,
    pub role_play_list: Option<Vec<String>>,
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name.bright_green())?;
        writeln!(f, "Display name: {}", self.display_name)?;
        writeln!(f, "Local name: {} @ {}", self.local_name, self.locale)?;
        writeln!(f, "Locale: {}", self.locale_name)?;
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "ID: {}", self.short_name)?;
        writeln!(f, "Voice type: {}", self.voice_type)?;
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Sample rate: {}Hz", self.sample_rate_hertz)?;
        writeln!(
            f,
            "Words per minute: {}",
            self.words_per_minute.as_deref().unwrap_or("N/A")
        )?;
        if let Some(style_list) = self.style_list.as_ref() {
            writeln!(f, "Styles: {style_list:?}")?;
        }
        if let Some(role_play_list) = self.role_play_list.as_ref() {
            writeln!(f, "Roles: {role_play_list:?}")?;
        }
        Ok(())
    }
}
