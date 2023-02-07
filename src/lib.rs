mod error;
mod msg;
mod ssml;
mod synthesizer;
mod types;
mod voice;

pub const ORIGIN: &str = "https://azure.microsoft.com";
pub const DEFAULT_ENDPOINT: &str =
    "wss://eastus.api.speech.microsoft.com/cognitiveservices/websocket/v1";

pub fn get_endpoint_by_region(region: &str) -> String {
    format!("wss://{region}.tts.speech.microsoft.com/cognitiveservices/websocket/v1")
}

pub use error::{AspeakError, Result};
use phf::phf_map;
pub use ssml::interpolate_ssml;
pub use synthesizer::{callback_play_blocking, Synthesizer, SynthesizerConfig};
pub use types::*;
pub use voice::Voice;

#[cfg(feature = "python")]
pub mod python;

pub type QualityMap = phf::Map<i8, AudioFormat>;

pub static WAV_QUALITY_MAP: QualityMap = phf_map! {
    -2i8 => AudioFormat::Riff8Khz16BitMonoPcm,
    -1i8 => AudioFormat::Riff16Khz16BitMonoPcm,
    0i8  => AudioFormat::Riff24Khz16BitMonoPcm,
    1i8  => AudioFormat::Riff24Khz16BitMonoPcm,
};

pub static MP3_QUALITY_MAP: QualityMap = phf_map! {
    -4i8 => AudioFormat::Audio16Khz32KBitRateMonoMp3,
    -3i8 => AudioFormat::Audio16Khz64KBitRateMonoMp3,
    -2i8 => AudioFormat::Audio16Khz128KBitRateMonoMp3,
    -1i8 => AudioFormat::Audio24Khz48KBitRateMonoMp3,
    0i8  => AudioFormat::Audio24Khz96KBitRateMonoMp3,
    1i8  => AudioFormat::Audio24Khz160KBitRateMonoMp3,
    2i8  => AudioFormat::Audio48Khz96KBitRateMonoMp3,
    3i8  => AudioFormat::Audio48Khz192KBitRateMonoMp3,
};

pub static OGG_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Ogg16Khz16BitMonoOpus,
    0i8  => AudioFormat::Ogg24Khz16BitMonoOpus,
    1i8  => AudioFormat::Ogg48Khz16BitMonoOpus,
};

pub static WEBM_QUALITY_MAP: QualityMap = phf_map! {
    -1i8 => AudioFormat::Webm16Khz16BitMonoOpus,
    0i8  => AudioFormat::Webm24Khz16BitMonoOpus,
    1i8  => AudioFormat::Webm24Khz16Bit24KbpsMonoOpus,
};

pub static QUALITY_MAP: phf::Map<&'static str, &'static QualityMap> = phf_map! {
    "wav" => &WAV_QUALITY_MAP,
    "mp3" => &MP3_QUALITY_MAP,
    "ogg" => &OGG_QUALITY_MAP,
    "webm" => &WEBM_QUALITY_MAP,
};

pub static DEFAULT_VOICES: phf::Map<&'static str, &'static str> = phf_map! {
    "af-ZA"=> "af-ZA-AdriNeural",
    "am-ET"=> "am-ET-AmehaNeural",
    "ar-AE"=> "ar-AE-FatimaNeural",
    "ar-BH"=> "ar-BH-AliNeural",
    "ar-DZ"=> "ar-DZ-AminaNeural",
    "ar-EG"=> "ar-EG-SalmaNeural",
    "ar-IQ"=> "ar-IQ-BasselNeural",
    "ar-JO"=> "ar-JO-SanaNeural",
    "ar-KW"=> "ar-KW-FahedNeural",
    "ar-LY"=> "ar-LY-ImanNeural",
    "ar-MA"=> "ar-MA-JamalNeural",
    "ar-QA"=> "ar-QA-AmalNeural",
    "ar-SA"=> "ar-SA-HamedNeural",
    "ar-SY"=> "ar-SY-AmanyNeural",
    "ar-TN"=> "ar-TN-HediNeural",
    "ar-YE"=> "ar-YE-MaryamNeural",
    "bg-BG"=> "bg-BG-BorislavNeural",
    "bn-BD"=> "bn-BD-NabanitaNeural",
    "bn-IN"=> "bn-IN-BashkarNeural",
    "ca-ES"=> "ca-ES-JoanaNeural",
    "cs-CZ"=> "cs-CZ-AntoninNeural",
    "cy-GB"=> "cy-GB-AledNeural",
    "da-DK"=> "da-DK-ChristelNeural",
    "de-AT"=> "de-AT-IngridNeural",
    "de-CH"=> "de-CH-JanNeural",
    "de-DE"=> "de-DE-KatjaNeural",
    "el-GR"=> "el-GR-AthinaNeural",
    "en-AU"=> "en-AU-NatashaNeural",
    "en-CA"=> "en-CA-ClaraNeural",
    "en-GB"=> "en-GB-LibbyNeural",
    "en-HK"=> "en-HK-SamNeural",
    "en-IE"=> "en-IE-ConnorNeural",
    "en-IN"=> "en-IN-NeerjaNeural",
    "en-KE"=> "en-KE-AsiliaNeural",
    "en-NG"=> "en-NG-AbeoNeural",
    "en-NZ"=> "en-NZ-MitchellNeural",
    "en-PH"=> "en-PH-JamesNeural",
    "en-SG"=> "en-SG-LunaNeural",
    "en-TZ"=> "en-TZ-ElimuNeural",
    "en-US"=> "en-US-JennyNeural",
    "en-ZA"=> "en-ZA-LeahNeural",
    "es-AR"=> "es-AR-ElenaNeural",
    "es-BO"=> "es-BO-MarceloNeural",
    "es-CL"=> "es-CL-CatalinaNeural",
    "es-CO"=> "es-CO-GonzaloNeural",
    "es-CR"=> "es-CR-JuanNeural",
    "es-CU"=> "es-CU-BelkysNeural",
    "es-DO"=> "es-DO-EmilioNeural",
    "es-EC"=> "es-EC-AndreaNeural",
    "es-ES"=> "es-ES-AlvaroNeural",
    "es-GQ"=> "es-GQ-JavierNeural",
    "es-GT"=> "es-GT-AndresNeural",
    "es-HN"=> "es-HN-CarlosNeural",
    "es-MX"=> "es-MX-DaliaNeural",
    "es-NI"=> "es-NI-FedericoNeural",
    "es-PA"=> "es-PA-MargaritaNeural",
    "es-PE"=> "es-PE-AlexNeural",
    "es-PR"=> "es-PR-KarinaNeural",
    "es-PY"=> "es-PY-MarioNeural",
    "es-SV"=> "es-SV-LorenaNeural",
    "es-US"=> "es-US-AlonsoNeural",
    "es-UY"=> "es-UY-MateoNeural",
    "es-VE"=> "es-VE-PaolaNeural",
    "et-EE"=> "et-EE-AnuNeural",
    "fa-IR"=> "fa-IR-DilaraNeural",
    "fi-FI"=> "fi-FI-SelmaNeural",
    "fil-PH"=> "fil-PH-AngeloNeural",
    "fr-BE"=> "fr-BE-CharlineNeural",
    "fr-CA"=> "fr-CA-SylvieNeural",
    "fr-CH"=> "fr-CH-ArianeNeural",
    "fr-FR"=> "fr-FR-DeniseNeural",
    "ga-IE"=> "ga-IE-ColmNeural",
    "gl-ES"=> "gl-ES-RoiNeural",
    "gu-IN"=> "gu-IN-DhwaniNeural",
    "he-IL"=> "he-IL-AvriNeural",
    "hi-IN"=> "hi-IN-MadhurNeural",
    "hr-HR"=> "hr-HR-GabrijelaNeural",
    "hu-HU"=> "hu-HU-NoemiNeural",
    "id-ID"=> "id-ID-ArdiNeural",
    "is-IS"=> "is-IS-GudrunNeural",
    "it-IT"=> "it-IT-IsabellaNeural",
    "ja-JP"=> "ja-JP-NanamiNeural",
    "jv-ID"=> "jv-ID-DimasNeural",
    "kk-KZ"=> "kk-KZ-AigulNeural",
    "km-KH"=> "km-KH-PisethNeural",
    "kn-IN"=> "kn-IN-GaganNeural",
    "ko-KR"=> "ko-KR-SunHiNeural",
    "lo-LA"=> "lo-LA-ChanthavongNeural",
    "lt-LT"=> "lt-LT-LeonasNeural",
    "lv-LV"=> "lv-LV-EveritaNeural",
    "mk-MK"=> "mk-MK-AleksandarNeural",
    "ml-IN"=> "ml-IN-MidhunNeural",
    "mr-IN"=> "mr-IN-AarohiNeural",
    "ms-MY"=> "ms-MY-OsmanNeural",
    "mt-MT"=> "mt-MT-GraceNeural",
    "my-MM"=> "my-MM-NilarNeural",
    "nb-NO"=> "nb-NO-PernilleNeural",
    "nl-BE"=> "nl-BE-ArnaudNeural",
    "nl-NL"=> "nl-NL-ColetteNeural",
    "pl-PL"=> "pl-PL-AgnieszkaNeural",
    "ps-AF"=> "ps-AF-GulNawazNeural",
    "pt-BR"=> "pt-BR-FranciscaNeural",
    "pt-PT"=> "pt-PT-DuarteNeural",
    "ro-RO"=> "ro-RO-AlinaNeural",
    "ru-RU"=> "ru-RU-SvetlanaNeural",
    "si-LK"=> "si-LK-SameeraNeural",
    "sk-SK"=> "sk-SK-LukasNeural",
    "sl-SI"=> "sl-SI-PetraNeural",
    "so-SO"=> "so-SO-MuuseNeural",
    "sr-RS"=> "sr-RS-NicholasNeural",
    "su-ID"=> "su-ID-JajangNeural",
    "sv-SE"=> "sv-SE-SofieNeural",
    "sw-KE"=> "sw-KE-RafikiNeural",
    "sw-TZ"=> "sw-TZ-DaudiNeural",
    "ta-IN"=> "ta-IN-PallaviNeural",
    "ta-LK"=> "ta-LK-KumarNeural",
    "ta-SG"=> "ta-SG-AnbuNeural",
    "te-IN"=> "te-IN-MohanNeural",
    "th-TH"=> "th-TH-PremwadeeNeural",
    "tr-TR"=> "tr-TR-AhmetNeural",
    "uk-UA"=> "uk-UA-OstapNeural",
    "ur-IN"=> "ur-IN-GulNeural",
    "ur-PK"=> "ur-PK-AsadNeural",
    "uz-UZ"=> "uz-UZ-MadinaNeural",
    "vi-VN"=> "vi-VN-HoaiMyNeural",
    "zh-CN"=> "zh-CN-XiaoxiaoNeural",
    "zh-HK"=> "zh-HK-HiuMaanNeural",
    "zh-TW"=> "zh-TW-HsiaoChenNeural",
    "zu-ZA"=> "zu-ZA-ThandoNeural",
};
