use super::block::Block;
use super::block_header::*;
use super::utils as mdf4_utils;

use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct Mdblock {
    header: BlockHeader,
    #[allow(dead_code)]
    md_data: String,
}

impl Mdblock {
    pub fn text(&self) -> String {
        self.clone().md_data
    }
}

impl Block for Mdblock {
    fn new() -> Self {
        Self {
            header: BlockHeader::create("##MD", 50, 0),
            md_data: "".to_string(),
        }
    }
    fn default() -> Self {
        Self {
            header: BlockHeader::create("##MD", 50, 0),
            md_data: "".to_string(),
        }
    }
    fn read(stream: &[u8], position: usize, little_endian: bool) -> (usize, Self) {
        let (pos, header) = BlockHeader::read(stream, position, little_endian);

        if !utils::eq(&header.id, "##MD".as_bytes()) {
            panic!("Error type incorrect");
        }

        let string_length = header.length as usize - header.byte_len();
        let md_data: String = mdf4_utils::str_from_u8(&stream[pos..(pos + string_length)]);

        ((pos + string_length), Self { header, md_data })
    }

    fn byte_len(&self) -> usize {
        24 + self.md_data.len() + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::MDF4::{block::Block, md_block::Mdblock};

    static RAW: [u8; 472] = [
        0x23, 0x23, 0x4D, 0x44, 0x00, 0x00, 0x00, 0x00, 0xD5, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x46, 0x48, 0x63, 0x6F, 0x6D,
        0x6D, 0x65, 0x6E, 0x74, 0x20, 0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3D, 0x27, 0x68, 0x74, 0x74,
        0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x61, 0x73, 0x61, 0x6D, 0x2E, 0x6E, 0x65,
        0x74, 0x2F, 0x6D, 0x64, 0x66, 0x2F, 0x76, 0x34, 0x27, 0x3E, 0x3C, 0x54, 0x58, 0x3E, 0x46,
        0x69, 0x6C, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
        0x2E, 0x3C, 0x2F, 0x54, 0x58, 0x3E, 0x3C, 0x74, 0x6F, 0x6F, 0x6C, 0x5F, 0x69, 0x64, 0x3E,
        0x54, 0x67, 0x74, 0x53, 0x76, 0x72, 0x3C, 0x2F, 0x74, 0x6F, 0x6F, 0x6C, 0x5F, 0x69, 0x64,
        0x3E, 0x3C, 0x74, 0x6F, 0x6F, 0x6C, 0x5F, 0x76, 0x65, 0x6E, 0x64, 0x6F, 0x72, 0x3E, 0x45,
        0x54, 0x41, 0x53, 0x20, 0x47, 0x6D, 0x62, 0x48, 0x3C, 0x2F, 0x74, 0x6F, 0x6F, 0x6C, 0x5F,
        0x76, 0x65, 0x6E, 0x64, 0x6F, 0x72, 0x3E, 0x3C, 0x74, 0x6F, 0x6F, 0x6C, 0x5F, 0x76, 0x65,
        0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E, 0x37, 0x2E, 0x32, 0x3C, 0x2F, 0x74, 0x6F, 0x6F, 0x6C,
        0x5F, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3E, 0x3C, 0x75, 0x73, 0x65, 0x72, 0x5F,
        0x6E, 0x61, 0x6D, 0x65, 0x3E, 0x77, 0x65, 0x6E, 0x39, 0x66, 0x65, 0x3C, 0x2F, 0x75, 0x73,
        0x65, 0x72, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x3E, 0x3C, 0x63, 0x6F, 0x6D, 0x6D, 0x6F, 0x6E,
        0x5F, 0x70, 0x72, 0x6F, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65, 0x73, 0x3E, 0x3C, 0x65, 0x20,
        0x6E, 0x61, 0x6D, 0x65, 0x3D, 0x22, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x69, 0x65, 0x73, 0x5F,
        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x22, 0x3E, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65,
        0x3A, 0x20, 0x6D, 0x64, 0x66, 0x34, 0x30, 0x70, 0x6C, 0x75, 0x67, 0x69, 0x6E, 0x2E, 0x64,
        0x6C, 0x6C, 0x2C, 0x20, 0x50, 0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x20, 0x76, 0x65, 0x72,
        0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x37, 0x2E, 0x32, 0x2E, 0x31, 0x30, 0x2E, 0x30, 0x2C,
        0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20,
        0x31, 0x37, 0x2E, 0x37, 0x32, 0x31, 0x30, 0x2E, 0x31, 0x35, 0x2E, 0x35, 0x32, 0x33, 0x38,
        0x30, 0x0D, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x3A, 0x20, 0x6D, 0x64, 0x66, 0x5F, 0x66,
        0x72, 0x61, 0x6D, 0x65, 0x77, 0x6F, 0x72, 0x6B, 0x2E, 0x64, 0x6C, 0x6C, 0x2C, 0x20, 0x50,
        0x72, 0x6F, 0x64, 0x75, 0x63, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A,
        0x20, 0x37, 0x2E, 0x32, 0x2E, 0x31, 0x30, 0x2E, 0x30, 0x2C, 0x20, 0x46, 0x69, 0x6C, 0x65,
        0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3A, 0x20, 0x31, 0x37, 0x2E, 0x37, 0x32,
        0x31, 0x30, 0x2E, 0x31, 0x35, 0x2E, 0x35, 0x32, 0x33, 0x38, 0x30, 0x0D, 0x3C, 0x2F, 0x65,
        0x3E, 0x3C, 0x2F, 0x63, 0x6F, 0x6D, 0x6D, 0x6F, 0x6E, 0x5F, 0x70, 0x72, 0x6F, 0x70, 0x65,
        0x72, 0x74, 0x69, 0x65, 0x73, 0x3E, 0x3C, 0x2F, 0x46, 0x48, 0x63, 0x6F, 0x6D, 0x6D, 0x65,
        0x6E, 0x74, 0x3E, 0x00, 0x40, 0x40, 0x40,
    ];

    #[test]
    fn md_read_test() {
        let (pos, md_block) = Mdblock::read(&RAW, 0, true);

        assert_eq!(469, pos);
        assert_eq!(469, md_block.byte_len());
    }
}