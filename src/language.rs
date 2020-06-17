/*
 * Copyright © 2020 Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::alphabet::Alphabet;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use serde::Deserialize;
use std::collections::HashSet;
use std::path::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, EnumIter, Eq, PartialEq, Hash)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Language {
    Afrikaans,
    Albanian,
    Arabic,
    Armenian,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Bulgarian,
    Catalan,
    Chinese,
    Croatian,
    Czech,
    Danish,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Georgian,
    German,
    Greek,
    Gujarati,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Kazakh,
    Korean,
    Latin,
    Latvian,
    Lithuanian,
    Macedonian,
    Malay,
    Marathi,
    Mongolian,
    Norwegian,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romanian,
    Russian,
    Serbian,
    Slovak,
    Slovene,
    Somali,
    Spanish,
    Swahili,
    Swedish,
    Tagalog,
    Tamil,
    Telugu,
    Thai,
    Turkish,
    Ukrainian,
    Urdu,
    Vietnamese,
    Welsh,
    Yoruba,
    Zulu,
}

impl Language {
    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        match self {
            Language::Afrikaans => IsoCode639_1::AF,
            Language::Albanian => IsoCode639_1::SQ,
            Language::Arabic => IsoCode639_1::AR,
            Language::Armenian => IsoCode639_1::HY,
            Language::Azerbaijani => IsoCode639_1::AZ,
            Language::Basque => IsoCode639_1::EU,
            Language::Belarusian => IsoCode639_1::BE,
            Language::Bengali => IsoCode639_1::BN,
            Language::Bosnian => IsoCode639_1::BS,
            Language::Bulgarian => IsoCode639_1::BG,
            Language::Catalan => IsoCode639_1::CA,
            Language::Chinese => IsoCode639_1::ZH,
            Language::Croatian => IsoCode639_1::HR,
            Language::Czech => IsoCode639_1::CS,
            Language::Danish => IsoCode639_1::DA,
            Language::Dutch => IsoCode639_1::NL,
            Language::English => IsoCode639_1::EN,
            Language::Esperanto => IsoCode639_1::EO,
            Language::Estonian => IsoCode639_1::ET,
            Language::Finnish => IsoCode639_1::FI,
            Language::French => IsoCode639_1::FR,
            Language::Georgian => IsoCode639_1::KA,
            Language::German => IsoCode639_1::DE,
            Language::Greek => IsoCode639_1::EL,
            Language::Gujarati => IsoCode639_1::GU,
            Language::Hebrew => IsoCode639_1::HE,
            Language::Hindi => IsoCode639_1::HI,
            Language::Hungarian => IsoCode639_1::HU,
            Language::Icelandic => IsoCode639_1::IS,
            Language::Indonesian => IsoCode639_1::ID,
            Language::Irish => IsoCode639_1::GA,
            Language::Italian => IsoCode639_1::IT,
            Language::Japanese => IsoCode639_1::JA,
            Language::Kazakh => IsoCode639_1::KK,
            Language::Korean => IsoCode639_1::KO,
            Language::Latin => IsoCode639_1::LA,
            Language::Latvian => IsoCode639_1::LV,
            Language::Lithuanian => IsoCode639_1::LT,
            Language::Macedonian => IsoCode639_1::MK,
            Language::Malay => IsoCode639_1::MS,
            Language::Marathi => IsoCode639_1::MR,
            Language::Mongolian => IsoCode639_1::MN,
            Language::Norwegian => IsoCode639_1::NO,
            Language::Persian => IsoCode639_1::FA,
            Language::Polish => IsoCode639_1::PL,
            Language::Portuguese => IsoCode639_1::PT,
            Language::Punjabi => IsoCode639_1::PA,
            Language::Romanian => IsoCode639_1::RO,
            Language::Russian => IsoCode639_1::RU,
            Language::Serbian => IsoCode639_1::SR,
            Language::Slovak => IsoCode639_1::SK,
            Language::Slovene => IsoCode639_1::SL,
            Language::Somali => IsoCode639_1::SO,
            Language::Spanish => IsoCode639_1::ES,
            Language::Swahili => IsoCode639_1::SW,
            Language::Swedish => IsoCode639_1::SV,
            Language::Tagalog => IsoCode639_1::TL,
            Language::Tamil => IsoCode639_1::TA,
            Language::Telugu => IsoCode639_1::TE,
            Language::Thai => IsoCode639_1::TH,
            Language::Turkish => IsoCode639_1::TR,
            Language::Ukrainian => IsoCode639_1::UK,
            Language::Urdu => IsoCode639_1::UR,
            Language::Vietnamese => IsoCode639_1::VI,
            Language::Welsh => IsoCode639_1::CY,
            Language::Yoruba => IsoCode639_1::YO,
            Language::Zulu => IsoCode639_1::ZU,
        }
    }

    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            Language::Afrikaans => IsoCode639_3::AFR,
            Language::Albanian => IsoCode639_3::SQI,
            Language::Arabic => IsoCode639_3::ARA,
            Language::Armenian => IsoCode639_3::HYE,
            Language::Azerbaijani => IsoCode639_3::AZE,
            Language::Basque => IsoCode639_3::EUS,
            Language::Belarusian => IsoCode639_3::BEL,
            Language::Bengali => IsoCode639_3::BEN,
            Language::Bosnian => IsoCode639_3::BOS,
            Language::Bulgarian => IsoCode639_3::BUL,
            Language::Catalan => IsoCode639_3::CAT,
            Language::Chinese => IsoCode639_3::ZHO,
            Language::Croatian => IsoCode639_3::HRV,
            Language::Czech => IsoCode639_3::CES,
            Language::Danish => IsoCode639_3::DAN,
            Language::Dutch => IsoCode639_3::NLD,
            Language::English => IsoCode639_3::ENG,
            Language::Esperanto => IsoCode639_3::EPO,
            Language::Estonian => IsoCode639_3::EST,
            Language::Finnish => IsoCode639_3::FIN,
            Language::French => IsoCode639_3::FRA,
            Language::Georgian => IsoCode639_3::KAT,
            Language::German => IsoCode639_3::DEU,
            Language::Greek => IsoCode639_3::ELL,
            Language::Gujarati => IsoCode639_3::GUJ,
            Language::Hebrew => IsoCode639_3::HEB,
            Language::Hindi => IsoCode639_3::HIN,
            Language::Hungarian => IsoCode639_3::HUN,
            Language::Icelandic => IsoCode639_3::ISL,
            Language::Indonesian => IsoCode639_3::IND,
            Language::Irish => IsoCode639_3::GLE,
            Language::Italian => IsoCode639_3::ITA,
            Language::Japanese => IsoCode639_3::JPN,
            Language::Kazakh => IsoCode639_3::KAZ,
            Language::Korean => IsoCode639_3::KOR,
            Language::Latin => IsoCode639_3::LAT,
            Language::Latvian => IsoCode639_3::LAV,
            Language::Lithuanian => IsoCode639_3::LIT,
            Language::Macedonian => IsoCode639_3::MKD,
            Language::Malay => IsoCode639_3::MSA,
            Language::Marathi => IsoCode639_3::MAR,
            Language::Mongolian => IsoCode639_3::MON,
            Language::Norwegian => IsoCode639_3::NOR,
            Language::Persian => IsoCode639_3::FAS,
            Language::Polish => IsoCode639_3::POL,
            Language::Portuguese => IsoCode639_3::POR,
            Language::Punjabi => IsoCode639_3::PAN,
            Language::Romanian => IsoCode639_3::RON,
            Language::Russian => IsoCode639_3::RUS,
            Language::Serbian => IsoCode639_3::SRP,
            Language::Slovak => IsoCode639_3::SLK,
            Language::Slovene => IsoCode639_3::SLV,
            Language::Somali => IsoCode639_3::SOM,
            Language::Spanish => IsoCode639_3::SPA,
            Language::Swahili => IsoCode639_3::SWA,
            Language::Swedish => IsoCode639_3::SWE,
            Language::Tagalog => IsoCode639_3::TGL,
            Language::Tamil => IsoCode639_3::TAM,
            Language::Telugu => IsoCode639_3::TEL,
            Language::Thai => IsoCode639_3::THA,
            Language::Turkish => IsoCode639_3::TUR,
            Language::Ukrainian => IsoCode639_3::UKR,
            Language::Urdu => IsoCode639_3::URD,
            Language::Vietnamese => IsoCode639_3::VIE,
            Language::Welsh => IsoCode639_3::CYM,
            Language::Yoruba => IsoCode639_3::YOR,
            Language::Zulu => IsoCode639_3::ZUL,
        }
    }

    pub fn alphabets(&self) -> HashSet<Alphabet> {
        match self {
            Language::Afrikaans
            | Language::Albanian
            | Language::Azerbaijani
            | Language::Basque
            | Language::Bosnian
            | Language::Catalan
            | Language::Croatian
            | Language::Czech
            | Language::Danish
            | Language::Dutch
            | Language::English
            | Language::Esperanto
            | Language::Estonian
            | Language::Finnish
            | Language::French
            | Language::German
            | Language::Hungarian
            | Language::Icelandic
            | Language::Indonesian
            | Language::Irish
            | Language::Italian
            | Language::Latin
            | Language::Latvian
            | Language::Lithuanian
            | Language::Malay
            | Language::Norwegian
            | Language::Polish
            | Language::Portuguese
            | Language::Romanian
            | Language::Slovak
            | Language::Slovene
            | Language::Somali
            | Language::Spanish
            | Language::Swahili
            | Language::Swedish
            | Language::Tagalog
            | Language::Turkish
            | Language::Vietnamese
            | Language::Welsh
            | Language::Yoruba
            | Language::Zulu => hashset!(Alphabet::Latin),

            Language::Belarusian
            | Language::Bulgarian
            | Language::Kazakh
            | Language::Macedonian
            | Language::Mongolian
            | Language::Russian
            | Language::Serbian
            | Language::Ukrainian => hashset!(Alphabet::Cyrillic),

            Language::Arabic | Language::Persian | Language::Urdu => hashset!(Alphabet::Arabic),

            Language::Hindi | Language::Marathi => hashset!(Alphabet::Devanagari),

            Language::Armenian => hashset!(Alphabet::Armenian),
            Language::Bengali => hashset!(Alphabet::Bengali),
            Language::Chinese => hashset!(Alphabet::Han),
            Language::Georgian => hashset!(Alphabet::Georgian),
            Language::Greek => hashset!(Alphabet::Greek),
            Language::Gujarati => hashset!(Alphabet::Gujarati),
            Language::Hebrew => hashset!(Alphabet::Hebrew),
            Language::Japanese => hashset!(Alphabet::Hiragana, Alphabet::Katakana, Alphabet::Han),
            Language::Korean => hashset!(Alphabet::Hangul),
            Language::Punjabi => hashset!(Alphabet::Gurmukhi),
            Language::Tamil => hashset!(Alphabet::Tamil),
            Language::Telugu => hashset!(Alphabet::Telugu),
            Language::Thai => hashset!(Alphabet::Thai),
        }
    }

    pub fn unique_characters(&self) -> &str {
        match self {
            Language::Albanian => "Ëë",
            Language::Azerbaijani => "Əə",
            Language::Catalan => "Ïï",
            Language::Czech => "ĚěŘřŮů",
            Language::Esperanto => "ĈĉĜĝĤĥĴĵŜŝŬŭ",
            Language::German => "ß",
            Language::Hungarian => "ŐőŰű",
            Language::Kazakh => "ӘәҒғҚқҢңҰұ",
            Language::Latvian => "ĢģĶķĻļŅņ",
            Language::Lithuanian => "ĖėĮįŲų",
            Language::Macedonian => "ЃѓЅѕЌќЏџ",
            Language::Marathi => "ळ",
            Language::Mongolian => "ӨөҮү",
            Language::Polish => "ŁłŃńŚśŹź",
            Language::Romanian => "Țţ",
            Language::Serbian => "ЂђЋћ",
            Language::Slovak => "ĹĺĽľŔŕ",
            Language::Spanish => "¿¡",
            Language::Ukrainian => "ҐґЄєЇї",
            Language::Vietnamese => "ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ",
            Language::Yoruba => "ŌōṢṣ",
            _ => "",
        }
    }
}