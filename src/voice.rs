use std::fmt::Display;

use colored::Colorize;
use serde::Deserialize;

/// Voice information
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Voice {
    display_name: String,
    gender: String,
    local_name: String,
    locale: String,
    locale_name: String,
    name: String,
    sample_rate_hertz: String,
    short_name: String,
    status: String,
    voice_type: String,
    words_per_minute: Option<String>,
    style_list: Option<Vec<String>>,
    role_play_list: Option<Vec<String>>,
}

impl Voice {
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn gender(&self) -> &str {
        &self.gender
    }

    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    pub fn locale(&self) -> &str {
        &self.locale
    }

    pub fn locale_name(&self) -> &str {
        &self.locale_name
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sample_rate_hertz(&self) -> &str {
        &self.sample_rate_hertz
    }

    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn voice_type(&self) -> &str {
        &self.voice_type
    }

    pub fn words_per_minute(&self) -> Option<&str> {
        self.words_per_minute.as_deref()
    }

    pub fn style_list(&self) -> Option<&[String]> {
        self.style_list.as_deref()
    }

    pub fn role_play_list(&self) -> Option<&[String]> {
        self.role_play_list.as_deref()
    }
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
