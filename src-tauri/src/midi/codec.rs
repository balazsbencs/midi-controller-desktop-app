/// 8-to-7 bit encode: every 7 input bytes → 8 output bytes.
/// MSBs of each input byte are collected into a leading byte.
/// Direct port of sysex_builder.c: sysex_encode_7bit / sysex_decode_7bit.
pub fn encode_7bit(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() * 8 / 7 + 2);
    let mut i = 0;
    while i < input.len() {
        let chunk = (input.len() - i).min(7);
        let mut msb: u8 = 0;
        for j in 0..chunk {
            if input[i + j] & 0x80 != 0 {
                msb |= 1 << j;
            }
        }
        out.push(msb);
        for j in 0..chunk {
            out.push(input[i + j] & 0x7F);
        }
        i += chunk;
    }
    out
}

pub fn decode_7bit(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() * 7 / 8 + 1);
    let mut i = 0;
    while i < input.len() {
        let msb = input[i];
        i += 1;
        for j in 0..7 {
            if i >= input.len() {
                break;
            }
            let hi = if msb & (1 << j) != 0 { 0x80 } else { 0x00 };
            out.push(input[i] | hi);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_all_zeros() {
        let data = vec![0u8; 14];
        assert_eq!(decode_7bit(&encode_7bit(&data)), data);
    }

    #[test]
    fn round_trip_all_ones() {
        let data = vec![0xFFu8; 21];
        assert_eq!(decode_7bit(&encode_7bit(&data)), data);
    }

    #[test]
    fn round_trip_ascending() {
        let data: Vec<u8> = (0..=127).collect();
        assert_eq!(decode_7bit(&encode_7bit(&data)), data);
    }

    #[test]
    fn round_trip_high_bytes() {
        let data = vec![0x80, 0xFF, 0xAB, 0xCD, 0x00, 0x7F, 0x81];
        assert_eq!(decode_7bit(&encode_7bit(&data)), data);
    }

    #[test]
    fn encoded_bytes_all_7bit_safe() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = encode_7bit(&data);
        for byte in &encoded {
            assert!(*byte <= 0x7F, "encoded byte {byte:#04x} exceeds 7-bit range");
        }
    }

    #[test]
    fn chunk_boundary_7_bytes() {
        let data = vec![0xAA; 7];
        let encoded = encode_7bit(&data);
        assert_eq!(encoded.len(), 8);
        assert_eq!(decode_7bit(&encoded), data);
    }

    #[test]
    fn partial_chunk() {
        let data = vec![0xAB, 0xCD];
        let encoded = encode_7bit(&data);
        assert_eq!(encoded.len(), 3);
        assert_eq!(decode_7bit(&encoded), data);
    }
}
