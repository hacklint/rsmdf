use crate::utils;

use super::block::{Block, LinkedBlock, DataBlock};
use super::block_header::*;
use super::mdf4_enums::ZipType;
use super::mdf4_file::link_extract;

#[derive(Debug, Clone, PartialEq)]
pub struct Hlblock {
    header: BlockHeader,
    #[allow(dead_code)]
    hl_dl_first: u64,
    #[allow(dead_code)]
    hl_flags: u16,
    #[allow(dead_code)]
    hl_zip_type: ZipType,
    hl_reserved: [u8; 5],
}

impl DataBlock for Hlblock {
    fn data_array(&self, _stream: &[u8], _little_endian: bool) -> Vec<u8> {        
        vec![1,2,3]  // Madness but just tring to get this to compile...
    }
}

impl LinkedBlock for Hlblock {
    fn next(&self, stream: &[u8], little_endian: bool) -> Option<Self> {
        if self.hl_dl_first == 0 {
            None
        } else {
            let (_pos, block) = Self::read(stream, self.hl_dl_first as usize, little_endian);
            Some(block)
        }
    }
    fn list(&self, stream: &[u8], little_endian: bool) -> Vec<Self> {
        let mut all = Vec::new();

        let next = self.next(stream, little_endian);

        all.push(self.clone());
        match next {
            None => {}
            Some(block) => all.append(&mut block.list(stream, little_endian)),
        }

        all

        // let next_block = self;

        // all.push(self.clone());
        // loop {
        //     let next_block = next_block.next(stream, little_endian);

        //     match next_block {
        //         Some(block) => all.push(block.clone()),
        //         None => break,
        //     }
        // }

        // all
    }
}


impl Block for Hlblock {
    fn new() -> Self {
        Self {
            header: BlockHeader::create("##HL", 50, 0),
            hl_dl_first: 0_u64,
            hl_flags: 0_u16,
            hl_zip_type: ZipType::Deflate,
            hl_reserved: [0_u8; 5],
        }
    }
    fn default() -> Self {
        Self {
            header: BlockHeader::create("##HL", 50, 0),
            hl_dl_first: 0_u64,
            hl_flags: 0_u16,
            hl_zip_type: ZipType::Deflate,
            hl_reserved: [0_u8; 5],
        }
    }
    fn read(stream: &[u8], position: usize, little_endian: bool) -> (usize, Self) {
        let (pos, header) = BlockHeader::read(stream, position, little_endian);

        if !utils::eq(&header.id, "##HL".as_bytes()) {
            panic!("Error HLBLOCK");
        }

        let (mut pos, mut address) = link_extract(stream, pos, little_endian, header.link_count);

        let hl_dl_first = address.remove(0);
        let hl_flags = utils::read(stream, little_endian, &mut pos);
        let hl_zip_type = ZipType::new(utils::read(stream, little_endian, &mut pos));
        let hl_reserved: [u8; 5] = utils::read(stream, little_endian, &mut pos);

        (
            pos,
            Self {
                header,
                hl_dl_first,
                hl_flags,
                hl_zip_type,
                hl_reserved,
            },
        )
    }

    fn byte_len(&self) -> usize {
        todo!()
    }
}
