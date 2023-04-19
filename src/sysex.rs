pub fn decode_sysex(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    for chunk in input[..].chunks_exact(8) {
        let mut mask = 0b0000_0001;
        let mut left_shift = 7;
        for element in &chunk[1..8] {
            output.push(*element | ((chunk[0] & mask) << left_shift));
            mask <<= 1;
            left_shift -= 1;
        }
    }
    output
}

#[allow(dead_code)]
pub fn encode_sysex(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let iter = input[..].chunks_exact(7);
    for chunk in iter {
        let mut control_byte = 0b0000_0000u8;
        let mut right_shift = 1;

        let mut result = Vec::new();

        for element in chunk {
            control_byte |= (element & 0b1000_0000u8) >> right_shift;
            result.push(element & 0b0111_1111u8);
            right_shift += 1;
        }

        /*if right_shift < 8 { // The chunk had less than 7 elements
            result.append(&mut vec![0;8-right_shift]);
        }*/
        output.push(control_byte);
        output.append(&mut result);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encoding_1() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let expected_output = [0, 1, 2, 3, 4, 5, 6, 7, 0, 8, 9, 10, 11, 12, 13, 14];
        assert_eq!(expected_output, &encode_sysex(&input)[0..16]);
    }

    #[test]
    fn test_basic_encoding_2() {
        let input = [
            128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141,
        ];
        let expected_output = [127, 0, 1, 2, 3, 4, 5, 6, 127, 7, 8, 9, 10, 11, 12, 13];
        assert_eq!(expected_output, &encode_sysex(&input)[0..16]);
    }

    #[test]
    fn test_basic_decoding() {
        let input = [
            0b0101_0101, // The 8th-bits for the next 7bit bytes
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            0b0000_0000,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
        ];
        assert_eq!(
            [129, 2, 131, 4, 133, 6, 135, 1, 2, 3, 4, 5, 6, 7],
            &decode_sysex(&input)[0..14]
        )
    }

    #[test]
    fn test_encode_decode() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let expected_output = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        assert_eq!(decode_sysex(&encode_sysex(&input)), expected_output);
    }
}
