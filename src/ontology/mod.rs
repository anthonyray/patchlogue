#[rustfmt::skip]
mod timbre_parameters;

use binrw::BinRead;
use serde::Serialize;
use std::str;
use timbre_parameters::TimbreParameters;

#[derive(Debug, PartialEq, BinRead, Serialize)]
#[br(little)]
#[br(magic = b"PROG")]
pub struct Program {
    #[br(try_map = |a: [u8; 12]| str::from_utf8(&a).map(|e| e.to_string()))]
    program_name: String,
    #[br(map = |oct: u8| (oct as i16) - 2i16)]
    octave: i16,
    #[br(map = |b: u8| b > 0u8)]
    sub_timbre_enabled: bool,
    edit_timbre: TimbreEdit,
    timbre_type: TimbreType,
    main_sub_balance: u8,
    #[br(pad_before = 1)]
    #[br(map = |b: u8| b > 0u8)]
    timbre_position_is_main_over_sub: bool,
    split_point: u8, // TODO : parse this into a more human readable struct
    tempo: u16,      // TODO : parse the type correctly.
    arp_target: ArpTarget,
    #[br(pad_before = 2)]
    category: Category,
    #[br(pad_before = 7)] // TODO: skipping Frequent Upper, Frequent Lower
    amp_velocity: u8,
    #[br(map = |b: u8| b>0u8)]
    portamento_mode: bool, // false: auto, true: On
    #[br(pad_before = 1)]
    program_level: u8, // Ranges between 12~132
    mod_effect_type: ModEffectType,
    mod_effect_speed: u16, // TODO : Looks like little endian
    mod_effect_depth: u16, // TODO : Looks little endian
    mod_effect_chorus: u8,
    mod_effect_ensemble: u8,
    mod_effect_phaser: u8,
    mod_effect_flanger: u8,
    mod_effect_user: u8,
    // TODO : Skipping microtuning
    // TODO: skipping scale key
    // TODO : skipping program tuning
    // TODO : skipping program transpose
    #[br(pad_before = 4)]
    arp_gate_time: u8, // TODO: ranges 1~73
    arp_rate: ArpRate,
    delay_reverb_dry_wet: u16,
    #[br(pad_before = 3)]
    delay_reverb_type: DelayReverbType,
    delay_reverb_time: u16,  // little endian ?
    delay_reverb_depth: u16, // little endain ?
    reverb_type: ReverbType,
    delay_type: DelayType,
    mod_effect_routing: TimbreRouting,
    delay_reverb_routing: TimbreRouting,
    #[br(map = |b: u8| b>0u8)]
    mod_effect_enabled: bool,
    #[br(map = |b: u8| b>0u8)]
    delay_reverb_effect_enabled: bool,
    arpeggiator_mode: ArpeggiatorMode,
    arpeggiator_range: u8,
    arpeggiator_type: ArpeggiatorType,
    // Skipping LIKE UPPER, LIKE LOWER
    #[br(pad_before = 4)]
    #[br(little)]
    pub timbre1: TimbreParameters,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum DelayReverbType {
    #[br(magic(0u8))]
    Off,
    #[br(magic(1u8))]
    Delay,
    #[br(magic(2u8))]
    Reverb,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum TimbreRouting {
    #[br(magic(1u8))]
    Main,
    #[br(magic(2u8))]
    Sub,
    #[br(magic(0u8))]
    MainSub,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum DelayType {
    #[br(magic(0u8))]
    Stereo,
    #[br(magic(1u8))]
    Mono,
    #[br(magic(2u8))]
    PingPong,
    #[br(magic(3u8))]
    Hipass,
    #[br(magic(4u8))]
    Tape,
    #[br(magic(5u8))]
    OneTap,
    #[br(magic(6u8))]
    StereoBPM,
    #[br(magic(7u8))]
    MonoBPM,
    #[br(magic(8u8))]
    PingBPM,
    #[br(magic(9u8))]
    HipassBPM,
    #[br(magic(10u8))]
    TapeBpm,
    #[br(magic(11u8))]
    Doubling,
    #[br(magic(12u8))]
    User1,
    #[br(magic(13u8))]
    User2,
    #[br(magic(14u8))]
    User3,
    #[br(magic(15u8))]
    User4,
    #[br(magic(16u8))]
    User5,
    #[br(magic(17u8))]
    User6,
    #[br(magic(18u8))]
    User7,
    #[br(magic(19u8))]
    User8,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ReverbType {
    #[br(magic(0u8))]
    Hall,
    #[br(magic(1u8))]
    Smooth,
    #[br(magic(2u8))]
    Arena,
    #[br(magic(3u8))]
    Plate,
    #[br(magic(4u8))]
    Room,
    #[br(magic(5u8))]
    EarlyRef,
    #[br(magic(6u8))]
    Space,
    #[br(magic(7u8))]
    Riser,
    #[br(magic(8u8))]
    Submarine,
    #[br(magic(9u8))]
    Horror,
    #[br(magic(10u8))]
    User1,
    #[br(magic(11u8))]
    User2,
    #[br(magic(12u8))]
    User3,
    #[br(magic(13u8))]
    User4,
    #[br(magic(14u8))]
    User5,
    #[br(magic(15u8))]
    User6,
    #[br(magic(16u8))]
    User7,
    #[br(magic(17u8))]
    User8,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ArpeggiatorType {
    #[br(magic(0u8))]
    Manual,
    #[br(magic(1u8))]
    Rise,
    #[br(magic(2u8))]
    Fall,
    #[br(magic(3u8))]
    RiseFall,
    #[br(magic(4u8))]
    Random,
    #[br(magic(5u8))]
    PolyRandom,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ArpeggiatorMode {
    #[br(magic(0u8))]
    Off,
    #[br(magic(1u8))]
    On,
    #[br(magic(2u8))]
    Latch,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ArpRate {
    #[br(magic(0u8))]
    Th64,
    #[br(magic(1u8))]
    Th48,
    #[br(magic(2u8))]
    Th32,
    #[br(magic(3u8))]
    Th24,
    #[br(magic(4u8))]
    Th16,
    #[br(magic(5u8))]
    Th16t,
    #[br(magic(6u8))]
    Th12,
    #[br(magic(7u8))]
    Th8,
    #[br(magic(8u8))]
    Th8t,
    #[br(magic(9u8))]
    Th6,
    #[br(magic(10u8))]
    Th4,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectUser {
    #[br(magic(0u8))]
    USER1,
    #[br(magic(1u8))]
    USER2,
    #[br(magic(2u8))]
    USER3,
    #[br(magic(3u8))]
    USER4,
    #[br(magic(4u8))]
    USER5,
    #[br(magic(5u8))]
    USER6,
    #[br(magic(6u8))]
    USER7,
    #[br(magic(7u8))]
    USER8,
    #[br(magic(8u8))]
    USER9,
    #[br(magic(9u8))]
    USER10,
    #[br(magic(10u8))]
    USER11,
    #[br(magic(11u8))]
    USER12,
    #[br(magic(12u8))]
    USER13,
    #[br(magic(13u8))]
    USER14,
    #[br(magic(14u8))]
    USER15,
    #[br(magic(15u8))]
    USER16,
}
#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectFlanger {
    #[br(magic(0u8))]
    Stereo,
    #[br(magic(1u8))]
    Light,
    #[br(magic(2u8))]
    Mono,
    #[br(magic(3u8))]
    HighSweep,
    #[br(magic(4u8))]
    MidSweep,
    #[br(magic(5u8))]
    PanSweep,
    #[br(magic(6u8))]
    MonoSweep,
    #[br(magic(7u8))]
    Triphase,
}
#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectPhaser {
    #[br(magic(0u8))]
    Stereo,
    #[br(magic(1u8))]
    Fast,
    #[br(magic(2u8))]
    Orange,
    #[br(magic(3u8))]
    Small,
    #[br(magic(4u8))]
    SmallReso,
    #[br(magic(5u8))]
    Black,
    #[br(magic(6u8))]
    Formant,
    #[br(magic(7u8))]
    Twinkle,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectEnsemble {
    #[br(magic(0u8))]
    Stereo,
    #[br(magic(1u8))]
    Light,
    #[br(magic(2u8))]
    Mono,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectChorus {
    #[br(magic(0u8))]
    Stereo,
    #[br(magic(1u8))]
    Light,
    #[br(magic(2u8))]
    Deep,
    #[br(magic(3u8))]
    Triphase,
    #[br(magic(4u8))]
    Harmonic,
    #[br(magic(5u8))]
    Mono,
    #[br(magic(6u8))]
    Feedback,
    #[br(magic(7u8))]
    Vibrato,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ModEffectType {
    #[br(magic(0u8))]
    Chorus,
    #[br(magic(1u8))]
    Ensemble,
    #[br(magic(2u8))]
    Phaser,
    #[br(magic(3u8))]
    Flanger,
    #[br(magic(4u8))]
    User,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum VoiceModeType {
    #[br(magic(0u8))]
    Poly,
    #[br(magic(1u8))]
    Mono,
    #[br(magic(2u8))]
    Unison,
    #[br(magic(3u8))]
    Chord,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum Category {
    #[br(magic(0u8))]
    PolySynth,
    #[br(magic(1u8))]
    Bass,
    #[br(magic(2u8))]
    Lead,
    #[br(magic(3u8))]
    PadStrings,
    #[br(magic(4u8))]
    KeyBell,
    #[br(magic(5u8))]
    Chord,
    #[br(magic(6u8))]
    Arp,
    #[br(magic(7u8))]
    Combination,
    #[br(magic(8u8))]
    Sfx,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum ArpTarget {
    #[br(magic(0u8))]
    MainPlusSub,
    #[br(magic(1u8))]
    Main,
    #[br(magic(2u8))]
    Sub,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum TimbreEdit {
    #[br(magic(0u8))]
    Main,
    #[br(magic(1u8))]
    MainPlusSub,
    #[br(magic(2u8))]
    Sub,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum TimbreType {
    #[br(magic(0u8))]
    Layer,
    #[br(magic(1u8))]
    XFade,
    #[br(magic(2u8))]
    Split,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum FilterCutOffDrive {
    #[br(magic(0u8))]
    Off,
    #[br(magic(1u8))]
    Mid,
    #[br(magic(2u8))]
    On,
}

#[derive(Debug, PartialEq, BinRead, Serialize)]
pub enum FilterCutOffKeyboardTracking {
    #[br(magic(0u8))]
    Off,
    #[br(magic(1u8))]
    Mid,
    #[br(magic(2u8))]
    On,
}
