const BLOCK_LENGTH: usize = 1048576;
const ALIGN_LENGTH: usize = 8;

pub fn calculate_hash(text: &str, lut: &[u8]) -> u32 {
    let mut result = 0u32;
    for b in text.bytes() {
        let temp = 1025 * (lut[b as usize] as u32 + result);
        result = (temp >> 6) ^ temp;
    }
    return 32769 * ((9 * result >> 11) ^ (9 * result));
}
