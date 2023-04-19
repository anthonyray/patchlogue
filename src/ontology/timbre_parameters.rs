use binrw::BinRead;
use serde::Serialize;

use crate::ontology::ModEffectUser;

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub struct TimbreParameters {
/* | +0    |       |  0~127  |*/  portamento_time : u8,      //                    
/* | +1    |       |         |*/  #[br(pad_before = 1)]     //                            
/* | +2    |       |  0~127  |*/  voice_spread : u8,         //                    
/* | +3    |       |         |*/  #[br(pad_before = 1)]     //                             
/* | +4    | H:0~7 |  0~1023 |*/  voice_mode_depth:u16,     //   *note P13 
/* | +5    | L:0~7 |         |*/                            //                
/* | +6    |       |  0~3    |*/  voice_mode_type: VoiceModeType,
                                  #[br(pad_before = 3)]
/* | +10   |       |  0~2    |*/  vco_1_wave: OscillatorWave,            //          *note P15 
/* | +11   |       |  0~3    |*/  vco_1_octave : u8,         //   0~3=2',4',8',16' 
                                  #[br(map = mappings::vco_pitch_mapping)]
/* | +12   | H:0~7 |  0~1023 |*/  vco_1_pitch:i32,              //      *note P16 
/* | +13   | L:0~7 |         |*/                            //                
/* | +14   | H:0~7 |  0~1023 |*/  vco_1_shape:u16,               //                
/* | +15   | L:0~7 |         |*/                            //                
/* | +16   |       |  0~2    |*/  pitch_eg_target: PitchEnveloppeGeneratorTarget,       //    0~2=VCO1,+,VCO2 
                                  #[br(map = mappings::pitch_eg_mapping)]
/* | +17   | H:0~7 |  0~1023 |*/  pitch_eg_int: i32,              //      *note P17 
/* | +18   | L:0~7 |         |*/                            //                
/* | +19   |       |  0~2    |*/  vco_2_wave: OscillatorWave,            //          *note P15 
/* | +20   |       |  0~3    |*/  vco_2_octave  : u8,        //   0~3=2',4',8',16' 
                                  #[br(map = mappings::vco_pitch_mapping)]
/* | +21   | H:0~7 |  0~1023 |*/  vco_2_pitch:i32,               //      *note P16 
/* | +22   | L:0~7 |         |*/                            //                
/* | +23   | H:0~7 |  0~1023 |*/  vco_2_shape:u16,               //                
/* | +24   | L:0~7 |         |*/                            //                
/* | +25   |       |  0~2    |*/  ring_sync : RingSync,            // 0~2=RING ON,OFF,SYNC ON 
/* | +26   | H:0~7 |  0~1023 |*/  cross_mod_depth:u16,           //                 
/* | +27   | L:0~7 |         |*/                            //                 
/* | +28   |       |  0,1    |*/  multi_routing : u8,        //  0,1=Pre VCF, Post VCF 
/* | +29   |       |  0~2    |*/  multi_type : MultiType,           //  0~2=NOISE,VPM,USER 
/* | +30   |       |  0~3    |*/  multi_octave : u8,         //    0~3=2',4',8',16' 
/* | +31   |       |  0~3    |*/  select_noise : SelectNoise,         //           *note P18 
/* | +32   |       |  0~15   |*/  select_vpm  : SelectVpm,          //           *note P19 
/* | +33   |       |  0~15   |*/  select_user : ModEffectUser,          //            *note P7 
/* | +34   | H:0~7 |  0~1023 |*/  shape_noise:u16,               //                
/* | +35   | L:0~7 |         |*/                            //                 
                                  #[br(pad_before = 2)]
/* | +38   | H:0~7 |  0~1023 |*/  vco1_level:u16,                //                 
/* | +39   | L:0~7 |         |*/                            //                 
/* | +40   | H:0~7 |  0~1023 |*/  vco2_level:u16,                //                 
/* | +41   | L:0~7 |         |*/                            //                 
/* | +42   | H:0~7 |  0~1023 |*/  multi_level:u16,               //                 
/* | +43   | L:0~7 |         |*/                            //                 
/* | +44   | H:0~7 |  0~1023 |*/  cutoff:u16,                    //                 
/* | +45   | L:0~7 |         |*/                            //                 
/* | +46   | H:0~7 |  0~1023 |*/  resonance:u16,                 //                 
/* | +47   | L:0~7 |         |*/                            //                 
                                  #[br(map = mappings::cutoff_eg_mapping)]
/* | +48   | H:0~7 |  0~1023 |*/  cutoff_eg_int:i64,             //       *note P20 
/* | +49   | L:0~7 |         |*/                            //                 
/* | +50   |       |  0~2    |*/  cutoff_drive   : ThreeStateSwitch,       //           *note P21 
                                  #[br(map = |b: u8| b > 0u8)]
/* | +51   |       |  0,1    |*/  low_cut_enabled   : bool,            //          0,1=OFF,ON 
/* | +52   |       |  0~2    |*/  cutoff_keyboard_track : ThreeStateSwitch,//           *note P21 
/* | +53   |       |  0~127  |*/  cutoff_velocity  : u8,   //                       
/* | +54   | H:0~7 |  0~1023 |*/  amp_eg_attack:u16,           //                   
/* | +55   | L:0~7 |         |*/                          //                   
/* | +56   | H:0~7 |  0~1023 |*/  amp_eg_decay:u16,            //                   
/* | +57   | L:0~7 |         |*/                          //                   
/* | +58   | H:0~7 |  0~1023 |*/  amp_eg_sustain:u16,          //                   
/* | +59   | L:0~7 |         |*/                          //                   
/* | +60   | H:0~7 |  0~1023 |*/  amp_eg_release:u16,          //                   
/* | +61   | L:0~7 |         |*/                          //                   
/* | +62   | H:0~7 |  0~1023 |*/  eg_attack:u16,               //                   
/* | +63   | L:0~7 |         |*/                          //                   
/* | +64   | H:0~7 |  0~1023 |*/  eg_decay:u16,                //                   
/* | +65   | L:0~7 |         |*/                          //                   
/* | +66   | H:0~7 |  0~1023 |*/  eg_sustain:u16,              //                   
/* | +67   | L:0~7 |         |*/                          //                   
/* | +68   | H:0~7 |  0~1023 |*/  eg_release:u16,             //                   
/* | +69   | L:0~7 |         |*/                          //                   
/* | +70   |       |  0~2    |*/  lfo_wave : OscillatorWave,           //             *note P15 
/* | +71   |       |  0~2    |*/  lfo_mode : LfoMode,           //     0~2=BPM,SLOW,FAST 
/* | +72   | H:0~7 |  0~1023 |*/  lfo_rate:u16,                //         *note P22 
/* | +73   | L:0~7 |         |*/                          //                   
/* | +74   | H:0~7 |  0~1023 |*/  lfo_int:u16,                 //                   
/* | +75   | L:0~7 |         |*/                          //                   
/* | +76   |       |  0~2    |*/  lfo_target : LfoTarget,         //0~2=CUTOFF,SHAPE,PITCH 
/* | +77   |       |  2~32   |*/  mod_wheel_assign : u8,   //             *note P23 
/* | +78   |       |  0~31   |*/  e_pedal_assign : u8,     //             *note P24 
/* | +79   |       |  0~12   |*/  bend_range_pos : u8,     //           OFF~+12Note 
/* | +80   |       |  0~12   |*/  bend_range_neg : u8,     //           OFF~-12Note 
/* | +81   |       |  0~200  |*/  vpm_engine_param1 : u8,  //     0~200=-100%~+100% 
/* | +82   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +83   |       |  0~200  |*/  vpm_engine_param2 : u8,  //     0~200=-100%~+100% 
/* | +84   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +85   |       |  0~200  |*/  vpm_engine_param3 : u8,  //     0~200=-100%~+100% 
/* | +86   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +87   |       |  0~200  |*/  vpm_engine_param4 : u8,  //     0~200=-100%~+100% 
/* | +88   |       |  0~200  |*/  vpm_engine_param5 : u8,  //     0~200=-100%~+100% 
                                  #[br(pad_before = 2)]
/* | +91   |       |  0~200  |*/  vpm_engine_param6 : u8,  //     0~200=-100%~+100% 
/* | +92   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +93   |       |         |*/  user_engine_param1 : u8, //             *note P25 
/* | +94   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +95   |       |         |*/  user_engine_param2 : u8, //             *note P25 
/* | +96   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +97   |       |         |*/  user_engine_param3 : u8, //             *note P25 
/* | +98   |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +99   |       |         |*/  user_engine_param4 : u8, //             *note P25 
/* | +100  |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +101  |       |         |*/  user_engine_param5 : u8, //             *note P25 
/* | +102  |       |         |*/  #[br(pad_before = 1)]   //                                
/* | +103  |       |         |*/  user_engine_param6 : u8, //             *note P25 
// /* | +104  |       |         |*/  #[br(pad_before = 1)]   //                                
// /* | +105  | 0~1   |         |*/  user_engine_param5_type //         *note P26 
// /* |       | 2~3   |         |*/  user_engine_param6_type //         *note P26 
// /* |       | 4~5   |         |*/  Reserved                //                   
// /* |       | 6~7   |         |*/  Reserved                //                   
// /* | +106  | 0~1   |         |*/  user_engine_param1_type //         *note P26 
// /* |       | 2~3   |         |*/  user_engine_param2_type //         *note P26 
// /* |       | 4~5   |         |*/  user_engine_param3_type //         *note P26 
// /* |       | 6~7   |         |*/  user_engine_param4_type //         *note P26 
/* | +107  | H:0~7 |  0~1023 |*/  shape_vpm: u16,           //                     
/* | +108  | L:0~7 |         |*/                          //                   
/* | +109  | H:0~7 |  0~1023 |*/  shift_shape_vpm : u16,    //                      
/* | +110  | L:0~7 |         |*/                          //                   
/* | +111  | H:0~7 |  0~1023 |*/  shape_user : u16,         //                       
/* | +112  | L:0~7 |         |*/                          //                   
/* | +113  | H:0~7 |  0~1023 |*/  shift_shape_user : u16,   //                       
/* | +114  | L:0~7 |         |*/                          //                   
/* | +115  |       |  0~200  |*/  mod_wheel_range: u8,         // 0~200=-100%~+100% 
/* | +116  |       |  0,1    |*/  lfo_key_sync : u8,           //        0,1=Off,On 
/* | +117  |       |  0,1    |*/  lfo_voice_sync : u8,         //        0,1=Off,On 
/* | +118  |       |  0~3    |*/  lfo_target_osc : u8,         //         *note P26 
/* | +119  |       |  0,1    |*/  mono_legato : u8,            //        0,1=Off,On 
                                  #[br(pad_after = 2)]
/* | +120  |       |  2~32   |*/  midi_after_touch : u8,       //         *note P23 
}  


#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum LfoTarget {
    #[br(magic(0u8))]
    Cutoff, 
    #[br(magic(1u8))]
    Shape, 
    #[br(magic(2u8))]
    Pitch
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum LfoMode {
    #[br(magic(0u8))]
    Bpm, 
    #[br(magic(1u8))]
    Slow, 
    #[br(magic(2u8))]
    Fast
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum ThreeStateSwitch {
    #[br(magic(0u8))]
    Low, 
    #[br(magic(1u8))]
    Mid, 
    #[br(magic(2u8))]
    High
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum SelectNoise { // 0~2=NOISE,VPM,USER 
    #[br(magic(0u8))]
    High,
    #[br(magic(1u8))]
    Low,
    #[br(magic(2u8))]
    Peak,
    #[br(magic(3u8))]
    Decim
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum SelectVpm { 
    #[br(magic(0u8))]
   Sin1,
   #[br(magic(1u8))]
   Sin2,
   #[br(magic(2u8))]
   Sin3,
   #[br(magic(3u8))]
   Sin4,
   #[br(magic(4u8))]
   Saw1,
   #[br(magic(5u8))]
   Saw2,
   #[br(magic(6u8))]
   Squ1,
   #[br(magic(7u8))]
   Squ2,
   #[br(magic(8u8))]
   Fat1,
   #[br(magic(9u8))]
   Fat2,
   #[br(magic(10u8))]
   Air1,
   #[br(magic(11u8))]
   Air2,
   #[br(magic(12u8))]
   Decay1,
   #[br(magic(13u8))]
   Decay2,
   #[br(magic(14u8))]
   Creep,
   #[br(magic(15u8))]
   Throat,
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum MultiType { // 0~2=NOISE,VPM,USER 
    #[br(magic(0u8))]
    Noise, 
    #[br(magic(1u8))]
    Vpm,
    #[br(magic(2u8))]
    User
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum RingSync { // 0~2=RING ON,OFF,SYNC ON 
    #[br(magic(0u8))]
    RingOn, 
    #[br(magic(1u8))]
    RingOff,
    #[br(magic(2u8))]
    Sync
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum PitchEnveloppeGeneratorTarget {
    #[br(magic(0u8))]
    Vco1, 
    #[br(magic(1u8))]
    Plus,
    #[br(magic(2u8))]
    Vco2
}

#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum OscillatorWave {
    #[br(magic(0u8))]
    Square, 
    #[br(magic(1u8))]
    Triangle,
    #[br(magic(2u8))]
    Saw
}


#[derive(Debug, PartialEq, Serialize, BinRead)]
pub enum VoiceModeType {
    #[br(magic(0u8))]
    Poly,
    #[br(magic(1u8))]
    Mono,
    #[br(magic(2u8))]
    Unison,
    #[br(magic(3u8))]
    Chord
}

mod mappings {
    fn linear_mapping(value: u16, input_start: i32, input_end:i32, output_start:i32, output_end:i32) -> i32 {
    let v = value as i32;
    output_start  + ((output_end - output_start) * ( (v - input_start) as f32 / ( input_end - input_start ) as f32) as i32)
}

pub fn vco_pitch_mapping(value:u16) -> i32 {
    match value {
        0 ..=3 => -1200,
        4 ..=355 => linear_mapping(value, 4, 356, -1200, -256), // -1200 ~ -256 
        356 ..=475 => linear_mapping(value, 356,476,-256,-16), //-256 ~  -16 
        476 ..=491 => linear_mapping(value, 476, 492,-16,0), //-16 ~    0 
        492 ..=531 =>  0, 
        532 ..=547 => linear_mapping(value, 532, 548, 0, 16),
        548 ..=667 => linear_mapping(value, 548,668,16,256), // 16 ~  256
        668 ..=1019 => linear_mapping(value, 668, 1020, 256, 1200), // 256 ~ 1200 
        1020 .. => 1200
    }
}

pub fn pitch_eg_mapping(value: u16) -> i32 {
        match value {
        0 ..=3 => -4800,
        4 ..=355 => linear_mapping(value, 4, 356, -4800, -1024),
        356 ..=475 => linear_mapping(value, 356,476,-1024,-64), 
        476 ..=491 => linear_mapping(value, 476, 492,-64,0),
        492 ..=531 =>  0, 
        532 ..=547 => linear_mapping(value, 532, 548, 0, 64),
        548 ..=667 => linear_mapping(value, 548,668,64,1024),
        668 ..=1019 => linear_mapping(value, 668, 1020, 1024, 4800),
        1020 .. => 4800
    }
}

pub fn cutoff_eg_mapping(value: u16) -> i64 {
        match value {
        0 ..=10 => -100,
        11 ..=490 => - ((492i64 - value as i64) * (492i64 - value as i64) * 4641i64 * 100i64) / 0x40000000,
        491 ..=531 => 0,
        532 ..=1012 => ((532i64 - value as i64) * (532i64 - value as i64) * 4641i64 * 100i64) / 0x40000000,
        1013 .. =>  100
    }
}
}  