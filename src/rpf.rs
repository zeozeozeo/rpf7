use crate::{RpfError, RpfResult};
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    ffi::CString,
    io::{Read, Seek, SeekFrom},
};

pub const RPF7_MAGIC: u32 = 0x52504637; // 'RPF7'
const ENCRYPTION_OPEN: u32 = 0x4e45504f; // 'OPEN'
const ENCRYPTION_AES: u32 = 0x0ffffff9;
const ENCRYPTION_NG: u32 = 0x0fefffff;
const ENTRY_SIZE: usize = 16; // 4x u32's

#[derive(Debug)]
pub enum EncryptionType {
    /// OpenIV style RPF with unencrypted TOC
    Open,
    /// AES encryption
    Aes,
    // NG encryption
    Ng,
}

impl EncryptionType {
    #[inline]
    pub const fn from_tag(tag: u32) -> Self {
        match tag {
            ENCRYPTION_OPEN => Self::Open,
            ENCRYPTION_AES => Self::Aes,
            ENCRYPTION_NG => Self::Ng,
            // should never get here, assume NG encryption
            _ => Self::Ng,
        }
    }
}

pub struct Rpf<R: ?Sized> {
    file: R,
}

fn read_c_string<R: Read>(reader: &mut R) -> RpfResult<String> {
    let mut buf = Vec::new();
    loop {
        let b = reader.read_u8()?;
        if b == 0 {
            return Ok(unsafe { CString::from_vec_unchecked(buf).into_string()? });
        }
        buf.push(b);
    }
}

impl<R: Read + Seek> Rpf<R> {
    #[inline]
    pub fn from_reader(file: R) -> Self {
        Self { file }
    }

    pub fn read_header(&mut self) -> RpfResult<()> {
        let version = self.file.read_u32::<LittleEndian>()?;
        if version != RPF7_MAGIC {
            return Err(RpfError::VersionMismatch(version));
        }

        let num_entries = self.file.read_u32::<LittleEndian>()?;
        let names_length = self.file.read_u32::<LittleEndian>()?;
        let encryption_type = EncryptionType::from_tag(self.file.read_u32::<LittleEndian>()?);
        log::info!("rpf: {num_entries} entries, names len: {names_length}, encryption: {encryption_type:?}");

        // read entry and name data
        let mut entries = vec![0; num_entries as usize * ENTRY_SIZE];
        self.file.read_exact(&mut entries)?;
        let mut names = vec![0; names_length as usize];
        self.file.read_exact(&mut names)?;

        for i in 0..num_entries {
            let y = self.file.read_u32::<LittleEndian>()?;
            let x = self.file.read_u32::<LittleEndian>()?;
            self.file.seek(SeekFrom::Current(-8))?;
            log::trace!("y: {y}, x: {x}");

            if x == 0x7fffff00 {
                log::trace!("reading directory entry {i}");
            } else if (x & 0x80000000) == 0 {
                log::trace!("reading binary file entry {i}");
            } else {
                log::trace!("reading resource file entry {i}");
            }
            let name_offset = self.file.read_u16::<LittleEndian>()?;
            log::trace!("name offset: {name_offset}");
            break;
        }

        Ok(())
    }
}
