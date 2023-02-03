use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Voice {
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
    words_per_minute: String,
    style_list: Option<Vec<String>>,
    role_play_list: Option<Vec<String>>,
}
