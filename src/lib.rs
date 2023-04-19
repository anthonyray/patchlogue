mod ontology;
mod sysex;

use binrw::BinRead;

use ontology::Program;
use std::io::Cursor;
use sysex::decode_sysex;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn read_program(input: &[u8]) -> String {
    let midi_data = &input[7..];
    let converted = decode_sysex(midi_data);
    let program = Program::read(&mut Cursor::new(converted)).unwrap(); // TODO : Unwrap
    serde_json::to_string(&program).unwrap() // TODO unwrap
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const PROGRAM_DATA: &'static [u8; 336] = include_bytes!("../resources/files/Prog_000.prog_bin");
    #[test]
    fn test_parses_program_data() {
        let program = Program::read(&mut Cursor::new(PROGRAM_DATA));
        assert!(program.is_ok());
    }

    const SYSEX_DATA: &'static [u8; 392] = include_bytes!("../resources/midi/basic.syx");
    #[ignore] // TODO
    #[test]
    fn test_round_trip_conversion() {
        let initial_sysex_midi_data = &SYSEX_DATA[7..];
        let decoded_midi_data = decode_sysex(initial_sysex_midi_data);
        let _program = Program::read(&mut Cursor::new(decoded_midi_data)).unwrap();
        // TODO : serialize program to bytes
        // encode it to sysex
        // compare it to initial_sysex_midi_data
        assert!(false);
    }
}
