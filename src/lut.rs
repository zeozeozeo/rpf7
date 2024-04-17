/// AES/NG keys and LUT storage.
pub struct LutStore {
    pub aes_key: Vec<u8>,
    pub ng_keys: Vec<Vec<u8>>,
    pub ng_decrypt_tables: [Vec<Vec<u8>>; 17],
    pub ng_encrypt_tables: [Vec<Vec<u8>>; 17],
    pub pc_lut: Vec<u8>,
}
