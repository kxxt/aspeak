mod cli;

use std::{
    error::Error,
    fs::File,
    io::{self, BufWriter, Cursor, Read, Write},
};

use cli::{Cli, Commands, InputArgs, OutputArgs};

use aspeak::{interpolate_ssml, AspeakError, Result, SynthesizerConfig, Voice, ORIGIN};
use clap::Parser;
use log::{debug, info};
use phf::phf_map;
use reqwest::header::{self, HeaderMap, HeaderValue};
use rodio::{Decoder, OutputStream, Sink};

fn process_input(args: InputArgs) -> Result<String> {
    let mut s = String::new();
    // todo: encoding
    if let Some(file) = args.file {
        File::open(&file)?.read_to_string(&mut s)?;
    } else {
        io::stdin().read_to_string(&mut s)?;
    }
    Ok(s)
}

fn process_output(args: OutputArgs) -> Result<Box<dyn FnMut(Option<&[u8]>) -> Result<()>>> {
    Ok(if let Some(file) = args.output {
        // todo: file already exists?
        let file = File::create(file)?;
        let mut buf_writer = BufWriter::new(file);
        Box::new(move |data| {
            Ok(if let Some(data) = data {
                buf_writer.write_all(data)?
            } else {
                buf_writer.flush()?
            })
        })
    } else {
        let mut buffer = Vec::new();
        Box::new(move |data| {
            if let Some(data) = data {
                buffer.extend_from_slice(data);
            } else {
                info!("Playing audio... ({} bytes)", buffer.len());
                let (_stream, stream_handle) = OutputStream::try_default()?;
                let sink = Sink::try_new(&stream_handle).unwrap();
                let cursor = Cursor::new(Vec::from(&buffer[..]));
                let source = Decoder::new(cursor)?;
                sink.append(source);
                sink.sleep_until_end();
            }
            Ok(())
        })
    })
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Commandline args: {cli:?}");
    // TODO: fix empty command case
    match cli.command.unwrap() {
        Commands::SSML {
            ssml,
            input_args,
            output_args,
        } => {
            let ssml = ssml
                .ok_or(AspeakError::InputError)
                .or_else(|_| process_input(input_args))?;
            let synthesizer =
                SynthesizerConfig::new(&cli.endpoint, output_args.format.unwrap()).connect()?; // todo
            let callback = process_output(output_args)?;
            synthesizer.synthesize(&ssml, callback)?;
        }
        Commands::Text {
            mut text_options,
            input_args,
            output_args,
        } => {
            text_options.text = Some(
                text_options
                    .text
                    .ok_or(AspeakError::InputError)
                    .or_else(|_| process_input(input_args))?,
            );
            text_options.locale = text_options.locale.or(Some("en-US".to_string()));
            text_options.voice = text_options.voice.or_else(|| {
                DEFAULT_VOICES
                    .get(text_options.locale.as_deref()?)
                    .map(|voice| voice.to_string())
            });
            let synthesizer =
                SynthesizerConfig::new(&cli.endpoint, output_args.format.unwrap()).connect()?;
            let ssml = interpolate_ssml(&text_options)?;
            let callback = process_output(output_args)?;
            synthesizer.synthesize(&ssml, callback)?;
        }
        Commands::ListVoices {
            ref voice,
            ref locale,
        } => {
            let url = format!("https://{}/cognitiveservices/voices/list", cli.endpoint);
            let headers =
                HeaderMap::from_iter([(header::ORIGIN, HeaderValue::from_str(ORIGIN).unwrap())]);
            let client = reqwest::blocking::ClientBuilder::new()
                .default_headers(headers)
                .build()
                .unwrap();
            let request = client.get(url).build()?;
            let voices = client.execute(request)?.json::<Vec<Voice>>()?;
            let voices = voices.iter();
            let locale_id = locale.as_deref();
            let voice_id = voice.as_deref();
            let voices: Box<dyn Iterator<Item = &Voice>> = {
                if locale_id.is_some() {
                    Box::new(voices.filter(|voice| Some(voice.locale.as_str()) == locale_id))
                } else if voice_id.is_some() {
                    Box::new(voices.filter(|voice| Some(voice.short_name.as_str()) == voice_id))
                } else {
                    Box::new(voices)
                }
            };
            for voice in voices {
                println!("{voice}");
            }
        }
        _ => todo!(),
    }

    Ok(())
}

static DEFAULT_VOICES: phf::Map<&'static str, &'static str> = phf_map! {
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
