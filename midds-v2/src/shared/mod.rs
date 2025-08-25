//! Shared types and utility types for MIDDS.
//!
//! This module contains common types used across all MIDDS implementations,
//! including party identification, date representation, language codes,
//! country codes, and musical keys. These types provide standardized
//! representations for metadata fields that are shared between different MIDDS entities.
//!
//! # Key Features
//!
//! - **Party Identification**: IPI and ISNI identifiers for music industry parties
//! - **Date**: Simple date representation without timezone complexity
//! - **Language**: Comprehensive language enum for internationalization
//! - **Country**: ISO 3166-1 alpha-2 country codes for global compatibility
//! - **Key**: Musical key notation including major/minor and enharmonic equivalents

use parity_scale_codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::MiddsString;

#[cfg(feature = "std")]
use ts_rs::TS;

#[cfg(feature = "std")]
const TS_DIR: &str = "shared/";

/// Beats per minute measurement type.
///
/// Used to represent the tempo of musical works and tracks.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Bpm;
///
/// let tempo: Bpm = 120; // 120 BPM
/// ```
pub type Bpm = u16;

/// Year representation type.
///
/// Used for creation years, recording years, and release dates.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Year;
///
/// let year: Year = 2024;
/// ```
pub type Year = u16;

/// Interested Party Information (IPI) identifier.
///
/// IPI is a unique identifier used in the music industry to identify
/// rightsholder parties such as songwriters, composers, and publishers.
/// IPI numbers are typically 9-11 digits.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Ipi;
///
/// let ipi: Ipi = 123456789; // Valid IPI number
/// ```
pub type Ipi = u64;

/// International Standard Name Identifier (ISNI).
///
/// ISNI is an ISO standard for uniquely identifying parties involved
/// in creative works. ISNI codes are 16 characters long.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Isni;
///
/// let isni: Isni = b"000000012345678X".to_vec().try_into().unwrap();
/// ```
pub type Isni = MiddsString<16>;

/// Flexible identifier for parties in the music industry.
///
/// This enum allows identification using either IPI, ISNI, or both identifiers,
/// providing maximum flexibility for different industry use cases.
///
/// # Examples
///
/// ```rust
/// use allfeat_midds_v2::shared::{PartyId, BothIdsContainer};
///
/// // Using only IPI
/// let party_ipi = PartyId::Ipi(123456789);
///
/// // Using only ISNI
/// let party_isni = PartyId::Isni(b"000000012345678X".to_vec().try_into().unwrap());
///
/// // Using both identifiers
/// let party_both = PartyId::Both(BothIdsContainer {
///     ipi: 123456789,
///     isni: b"000000012345678X".to_vec().try_into().unwrap(),
/// });
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub enum PartyId {
    /// Party identified by IPI number only.
    Ipi(Ipi),
    /// Party identified by ISNI code only.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    Isni(Isni),
    /// Party identified by both IPI and ISNI.
    Both(BothIdsContainer),
}

/// Container for parties that have both IPI and ISNI identifiers.
///
/// This struct is used within [`PartyId::Both`] to hold both identifier types
/// when a party is known by both systems.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::BothIdsContainer;
///
/// let container = BothIdsContainer {
///     ipi: 123456789,
///     isni: b"000000012345678X".to_vec().try_into().unwrap(),
/// };
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub struct BothIdsContainer {
    /// The IPI identifier for this party.
    pub ipi: Ipi,
    /// The ISNI identifier for this party.
    #[cfg_attr(feature = "std", ts(as = "String"))]
    pub isni: Isni,
}

/// Generated music genres module
#[midds_v2_codegen::music_genres(path = "./music-genres.json")]
pub mod genres {}

/// Representation of a date for use in MIDDS fields.
///
/// This struct contains the year, month, and day in numerical format.
/// It is meant for simple, unambiguous date representation without timezone or time information.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Date;
///
/// let release_date = Date {
///     year: 2024,
///     month: 6,
///     day: 15,
/// };
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

/// Enum representing the language in which MIDDS metadata is written.
///
/// This is used to identify the language context of the metadata fields.
/// Supports major world languages used in the music industry.
///
/// # Example
///
/// ```rust
/// use allfeat_midds_v2::shared::Language;
///
/// let song_language = Language::English;
/// let french_song = Language::French;
/// ```
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Debug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub enum Language {
    English = 0,
    French = 1,
    Spanish = 2,
    German = 3,
    Italian = 4,
    Portuguese = 5,
    Russian = 6,
    Chinese = 7,
    Japanese = 8,
    Korean = 9,
    Arabic = 10,
    Hindi = 11,
    Dutch = 12,
    Swedish = 13,
    Norwegian = 14,
    Finnish = 15,
    Polish = 16,
    Turkish = 17,
    Hebrew = 18,
    Greek = 19,
    Latin = 20,
    Esperanto = 21,
}

/// Enum representing the ISO 3166-1 alpha-2 country codes.
///
/// This enum includes all officially recognized countries and territories.
/// Each variant corresponds to a two-letter country code.
#[repr(u16)]
#[derive(
    Clone,
    Copy,
    Debug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub enum Country {
    /// Andorra
    AD,
    /// United Arab Emirates
    AE,
    /// Afghanistan
    AF,
    /// Antigua and Barbuda
    AG,
    /// Anguilla
    AI,
    /// Albania
    AL,
    /// Armenia
    AM,
    /// Angola
    AO,
    /// Antarctica
    AQ,
    /// Argentina
    AR,
    /// American Samoa
    AS,
    /// Austria
    AT,
    /// Australia
    AU,
    /// Aruba
    AW,
    /// Åland Islands
    AX,
    /// Azerbaijan
    AZ,
    /// Bosnia and Herzegovina
    BA,
    /// Barbados
    BB,
    /// Bangladesh
    BD,
    /// Belgium
    BE,
    /// Burkina Faso
    BF,
    /// Bulgaria
    BG,
    /// Bahrain
    BH,
    /// Burundi
    BI,
    /// Benin
    BJ,
    /// Saint Barthélemy
    BL,
    /// Bermuda
    BM,
    /// Brunei Darussalam
    BN,
    /// Bolivia, Plurinational State of
    BO,
    /// Bonaire, Sint Eustatius and Saba
    BQ,
    /// Brazil
    BR,
    /// Bahamas
    BS,
    /// Bhutan
    BT,
    /// Bouvet Island
    BV,
    /// Botswana
    BW,
    /// Belarus
    BY,
    /// Belize
    BZ,
    /// Canada
    CA,
    /// Cocos (Keeling) Islands
    CC,
    /// Congo, The Democratic Republic of the
    CD,
    /// Central African Republic
    CF,
    /// Congo
    CG,
    /// Switzerland
    CH,
    /// Côte d'Ivoire
    CI,
    /// Cook Islands
    CK,
    /// Chile
    CL,
    /// Cameroon
    CM,
    /// China
    CN,
    /// Colombia
    CO,
    /// Costa Rica
    CR,
    /// Cuba
    CU,
    /// Cabo Verde
    CV,
    /// Curaçao
    CW,
    /// Christmas Island
    CX,
    /// Cyprus
    CY,
    /// Czechia
    CZ,
    /// Germany
    DE,
    /// Djibouti
    DJ,
    /// Denmark
    DK,
    /// Dominica
    DM,
    /// Dominican Republic
    DO,
    /// Algeria
    DZ,
    /// Ecuador
    EC,
    /// Estonia
    EE,
    /// Egypt
    EG,
    /// Western Sahara
    EH,
    /// Eritrea
    ER,
    /// Spain
    ES,
    /// Ethiopia
    ET,
    /// Finland
    FI,
    /// Fiji
    FJ,
    /// Falkland Islands (Malvinas)
    FK,
    /// Micronesia, Federated States of
    FM,
    /// Faroe Islands
    FO,
    /// France
    FR,
    /// Gabon
    GA,
    /// United Kingdom
    GB,
    /// Grenada
    GD,
    /// Georgia
    GE,
    /// French Guiana
    GF,
    /// Guernsey
    GG,
    /// Ghana
    GH,
    /// Gibraltar
    GI,
    /// Greenland
    GL,
    /// Gambia
    GM,
    /// Guinea
    GN,
    /// Guadeloupe
    GP,
    /// Equatorial Guinea
    GQ,
    /// Greece
    GR,
    /// South Georgia and the South Sandwich Islands
    GS,
    /// Guatemala
    GT,
    /// Guam
    GU,
    /// Guinea-Bissau
    GW,
    /// Guyana
    GY,
    /// Hong Kong
    HK,
    /// Heard Island and `McDonald` Islands
    HM,
    /// Honduras
    HN,
    /// Croatia
    HR,
    /// Haiti
    HT,
    /// Hungary
    HU,
    /// Indonesia
    ID,
    /// Ireland
    IE,
    /// Israel
    IL,
    /// Isle of Man
    IM,
    /// India
    IN,
    /// British Indian Ocean Territory
    IO,
    /// Iraq
    IQ,
    /// Iran, Islamic Republic of
    IR,
    /// Iceland
    IS,
    /// Italy
    IT,
    /// Jersey
    JE,
    /// Jamaica
    JM,
    /// Jordan
    JO,
    /// Japan
    JP,
    /// Kenya
    KE,
    /// Kyrgyzstan
    KG,
    /// Cambodia
    KH,
    /// Kiribati
    KI,
    /// Comoros
    KM,
    /// Saint Kitts and Nevis
    KN,
    /// Korea, Democratic People's Republic of
    KP,
    /// Korea, Republic of
    KR,
    /// Kuwait
    KW,
    /// Cayman Islands
    KY,
    /// Kazakhstan
    KZ,
    /// Lao People's Democratic Republic
    LA,
    /// Lebanon
    LB,
    /// Saint Lucia
    LC,
    /// Liechtenstein
    LI,
    /// Sri Lanka
    LK,
    /// Liberia
    LR,
    /// Lesotho
    LS,
    /// Lithuania
    LT,
    /// Luxembourg
    LU,
    /// Latvia
    LV,
    /// Libya
    LY,
    /// Morocco
    MA,
    /// Monaco
    MC,
    /// Moldova, Republic of
    MD,
    /// Montenegro
    ME,
    /// Saint Martin (French part)
    MF,
    /// Madagascar
    MG,
    /// Marshall Islands
    MH,
    /// North Macedonia
    MK,
    /// Mali
    ML,
    /// Myanmar
    MM,
    /// Mongolia
    MN,
    /// Macao
    MO,
    /// Northern Mariana Islands
    MP,
    /// Martinique
    MQ,
    /// Mauritania
    MR,
    /// Montserrat
    MS,
    /// Malta
    MT,
    /// Mauritius
    MU,
    /// Maldives
    MV,
    /// Malawi
    MW,
    /// Mexico
    MX,
    /// Malaysia
    MY,
    /// Mozambique
    MZ,
    /// Namibia
    NA,
    /// New Caledonia
    NC,
    /// Niger
    NE,
    /// Norfolk Island
    NF,
    /// Nigeria
    NG,
    /// Nicaragua
    NI,
    /// Netherlands
    NL,
    /// Norway
    NO,
    /// Nepal
    NP,
    /// Nauru
    NR,
    /// Niue
    NU,
    /// New Zealand
    NZ,
    /// Oman
    OM,
    /// Panama
    PA,
    /// Peru
    PE,
    /// French Polynesia
    PF,
    /// Papua New Guinea
    PG,
    /// Philippines
    PH,
    /// Pakistan
    PK,
    /// Poland
    PL,
    /// Saint Pierre and Miquelon
    PM,
    /// Pitcairn
    PN,
    /// Puerto Rico
    PR,
    /// Palestine, State of
    PS,
    /// Portugal
    PT,
    /// Palau
    PW,
    /// Paraguay
    PY,
    /// Qatar
    QA,
    /// Réunion
    RE,
    /// Romania
    RO,
    /// Serbia
    RS,
    /// Russian Federation
    RU,
    /// Rwanda
    RW,
    /// Saudi Arabia
    SA,
    /// Solomon Islands
    SB,
    /// Seychelles
    SC,
    /// Sudan
    SD,
    /// Sweden
    SE,
    /// Singapore
    SG,
    /// Saint Helena, Ascension and Tristan da Cunha
    SH,
    /// Slovenia
    SI,
    /// Svalbard and Jan Mayen
    SJ,
    /// Slovakia
    SK,
    /// Sierra Leone
    SL,
    /// San Marino
    SM,
    /// Senegal
    SN,
    /// Somalia
    SO,
    /// Suriname
    SR,
    /// South Sudan
    SS,
    /// Sao Tome and Principe
    ST,
    /// El Salvador
    SV,
    /// Sint Maarten (Dutch part)
    SX,
    /// Syrian Arab Republic
    SY,
    /// Eswatini
    SZ,
    /// Turks and Caicos Islands
    TC,
    /// Chad
    TD,
    /// French Southern Territories
    TF,
    /// Togo
    TG,
    /// Thailand
    TH,
    /// Tajikistan
    TJ,
    /// Tokelau
    TK,
    /// Timor-Leste
    TL,
    /// Turkmenistan
    TM,
    /// Tunisia
    TN,
    /// Tonga
    TO,
    /// Turkey
    TR,
    /// Trinidad and Tobago
    TT,
    /// Tuvalu
    TV,
    /// Taiwan, Province of China
    TW,
    /// Tanzania, United Republic of
    TZ,
    /// Ukraine
    UA,
    /// Uganda
    UG,
    /// United States Minor Outlying Islands
    UM,
    /// United States
    US,
    /// Uruguay
    UY,
    /// Uzbekistan
    UZ,
    /// Holy See (Vatican City State)
    VA,
    /// Saint Vincent and the Grenadines
    VC,
    /// Venezuela, Bolivarian Republic of
    VE,
    /// Virgin Islands, British
    VG,
    /// Virgin Islands, U.S.
    VI,
    /// Viet Nam
    VN,
    /// Vanuatu
    VU,
    /// Wallis and Futuna
    WF,
    /// Samoa
    WS,
    /// Yemen
    YE,
    /// Mayotte
    YT,
    /// South Africa
    ZA,
    /// Zambia
    ZM,
    /// Zimbabwe
    ZW,
}

/// Enum representing all major and minor keys, including sharps, flats,
/// and their enharmonic equivalents.
///
/// This can be used to specify the musical key of a track or composition
/// with precise notation.
///
/// # Naming Convention
///
/// - `m` suffix indicates minor keys
/// - `s` suffix indicates sharp (#)
/// - `b` suffix indicates flat (♭)
/// - Enharmonic equivalents are preserved for exact notation
///
/// # Examples
///
/// ```rust
/// use allfeat_midds_v2::shared::Key;
///
/// let c_major = Key::C;
/// let a_minor = Key::Am;
/// let f_sharp_major = Key::Fs;
/// let g_flat_major = Key::Gb; // Enharmonic equivalent of F#
/// ```
#[repr(u8)]
#[derive(
    Clone,
    Copy,
    Debug,
    Encode,
    Decode,
    PartialEq,
    Eq,
    DecodeWithMemTracking,
    TypeInfo,
    MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(TS))]
#[cfg_attr(feature = "std", ts(export))]
#[cfg_attr(feature = "std", ts(export_to = TS_DIR))]
pub enum Key {
    A = 0,
    Am = 1,
    As = 2,  // A#
    Asm = 3, // A#m
    Ab = 4,
    Abm = 5,
    B = 6,
    Bm = 7,
    Bs = 8,  // B#
    Bsm = 9, // B#m
    Bb = 10,
    Bbm = 11,
    C = 12,
    Cm = 13,
    Cs = 14,  // C#
    Csm = 15, // C#m
    Cb = 16,
    Cbm = 17,
    D = 18,
    Dm = 19,
    Ds = 20,  // D#
    Dsm = 21, // D#m
    Db = 22,
    Dbm = 23,
    E = 24,
    Em = 25,
    Es = 26,  // E#
    Esm = 27, // E#m
    Eb = 28,
    Ebm = 29,
    F = 30,
    Fm = 31,
    Fs = 32,  // F#
    Fsm = 33, // F#m
    Fb = 34,
    Fbm = 35,
    G = 36,
    Gm = 37,
    Gs = 38,  // G#
    Gsm = 39, // G#m
    Gb = 40,
    Gbm = 41,
}
