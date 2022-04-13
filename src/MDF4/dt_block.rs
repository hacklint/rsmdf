use super::block::Block;
use super::block_header::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Dtblock {
    header: BlockHeader,
    dt_data: Vec<u8>,
}

impl Block for Dtblock {
    fn new() -> Self {
        Self {
            header: BlockHeader::create("##DT", 24, 0),
            dt_data: Vec::new(),
        }
    }

    fn default() -> Self {
        Self {
            header: BlockHeader::new(),
            dt_data: Vec::new(),
        }
    }

    fn read(stream: &[u8], position: usize, little_endian: bool) -> (usize, Self) {
        let (pos, header) = BlockHeader::read(stream, position, little_endian);

        let dt_data = stream[pos..(pos + header.length as usize - header.byte_len())].to_vec();

        let pos = pos + dt_data.len();

        (pos, Self { header, dt_data })
    }

    fn byte_len(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::MDF4::block::Block;

    use super::Dtblock;

    static RAW: [u8; 1144] = [
        0x23, 0x23, 0x44, 0x54, 0x00, 0x00, 0x00, 0x00, 0x74, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x19, 0x60, 0x9C, 0xAE, 0xDD,
        0xBC, 0x3F, 0x00, 0x82, 0xDC, 0xFC, 0x1A, 0xA4, 0x3B, 0xCB, 0x3F, 0x0A, 0x41, 0xD6, 0xE4,
        0x73, 0x38, 0x04, 0xD4, 0x3F, 0x14, 0x41, 0x3E, 0x4B, 0xDA, 0x9E, 0x6A, 0xDA, 0x3F, 0x1E,
        0x20, 0xD3, 0x58, 0xA0, 0x82, 0x68, 0xE0, 0x3F, 0x28, 0x20, 0x07, 0x8C, 0xD3, 0xB5, 0x9B,
        0xE3, 0x3F, 0x32, 0x20, 0x3B, 0xBF, 0x06, 0xE9, 0xCE, 0xE6, 0x3F, 0x3C, 0x20, 0x6F, 0xF2,
        0x39, 0x1C, 0x02, 0xEA, 0x3F, 0x46, 0x20, 0xA3, 0x25, 0x6D, 0x4F, 0x35, 0xED, 0x3F, 0x50,
        0x90, 0x6B, 0x2C, 0x50, 0x41, 0x34, 0xF0, 0x3F, 0x5A, 0x90, 0x05, 0xC6, 0xE9, 0xDA, 0xCD,
        0xF1, 0x3F, 0x64, 0x90, 0x9F, 0x5F, 0x83, 0x74, 0x67, 0xF3, 0x3F, 0x6E, 0x90, 0x39, 0xF9,
        0x1C, 0x0E, 0x01, 0xF5, 0x3F, 0x78, 0x90, 0xD3, 0x92, 0xB6, 0xA7, 0x9A, 0xF6, 0x3F, 0x82,
        0x90, 0x6D, 0x2C, 0x50, 0x41, 0x34, 0xF8, 0x3F, 0x8C, 0x90, 0x07, 0xC6, 0xE9, 0xDA, 0xCD,
        0xF9, 0x3F, 0x96, 0x90, 0xA1, 0x5F, 0x83, 0x74, 0x67, 0xFB, 0x3F, 0xA0, 0x90, 0x3B, 0xF9,
        0x1C, 0x0E, 0x01, 0xFD, 0x3F, 0xAA, 0x90, 0xD5, 0x92, 0xB6, 0xA7, 0x9A, 0xFE, 0x3F, 0xB4,
        0xC8, 0x37, 0x16, 0xA8, 0x20, 0x1A, 0x00, 0x40, 0xBE, 0xC8, 0x04, 0xE3, 0x74, 0xED, 0xE6,
        0x00, 0x40, 0xC8, 0xC8, 0xD1, 0xAF, 0x41, 0xBA, 0xB3, 0x01, 0x40, 0xD2, 0xC8, 0x9E, 0x7C,
        0x0E, 0x87, 0x80, 0x02, 0x40, 0xDC, 0xC8, 0x6B, 0x49, 0xDB, 0x53, 0x4D, 0x03, 0x40, 0xE6,
        0xC8, 0x38, 0x16, 0xA8, 0x20, 0x1A, 0x04, 0x40, 0xF0, 0xC8, 0x05, 0xE3, 0x74, 0xED, 0xE6,
        0x04, 0x40, 0xFA, 0xC8, 0xD2, 0xAF, 0x41, 0xBA, 0xB3, 0x05, 0x40, 0x04, 0xC8, 0x9F, 0x7C,
        0x0E, 0x87, 0x80, 0x06, 0x40, 0x0E, 0xC8, 0x6C, 0x49, 0xDB, 0x53, 0x4D, 0x07, 0x40, 0x18,
        0xC8, 0x39, 0x16, 0xA8, 0x20, 0x1A, 0x08, 0x40, 0x22, 0xC8, 0x06, 0xE3, 0x74, 0xED, 0xE6,
        0x08, 0x40, 0x2C, 0xC8, 0xD3, 0xAF, 0x41, 0xBA, 0xB3, 0x09, 0x40, 0x36, 0xC8, 0xA0, 0x7C,
        0x0E, 0x87, 0x80, 0x0A, 0x40, 0x40, 0xC8, 0x6D, 0x49, 0xDB, 0x53, 0x4D, 0x0B, 0x40, 0x4A,
        0xC8, 0x3A, 0x16, 0xA8, 0x20, 0x1A, 0x0C, 0x40, 0x54, 0xC8, 0x07, 0xE3, 0x74, 0xED, 0xE6,
        0x0C, 0x40, 0x5E, 0xC8, 0xD4, 0xAF, 0x41, 0xBA, 0xB3, 0x0D, 0x40, 0x68, 0xC8, 0xA1, 0x7C,
        0x0E, 0x87, 0x80, 0x0E, 0x40, 0x72, 0xC8, 0x6E, 0x49, 0xDB, 0x53, 0x4D, 0x0F, 0x40, 0x7C,
        0xE4, 0x1D, 0x0B, 0x54, 0x10, 0x0D, 0x10, 0x40, 0x86, 0x64, 0x84, 0x71, 0xBA, 0x76, 0x73,
        0x10, 0x40, 0x90, 0xE4, 0xEA, 0xD7, 0x20, 0xDD, 0xD9, 0x10, 0x40, 0x9A, 0x64, 0x51, 0x3E,
        0x87, 0x43, 0x40, 0x11, 0x40, 0xA4, 0xE4, 0xB7, 0xA4, 0xED, 0xA9, 0xA6, 0x11, 0x40, 0xAE,
        0x64, 0x1E, 0x0B, 0x54, 0x10, 0x0D, 0x12, 0x40, 0xB8, 0xE4, 0x84, 0x71, 0xBA, 0x76, 0x73,
        0x12, 0x40, 0xC2, 0x64, 0xEB, 0xD7, 0x20, 0xDD, 0xD9, 0x12, 0x40, 0xCC, 0xE4, 0x51, 0x3E,
        0x87, 0x43, 0x40, 0x13, 0x40, 0xD6, 0x64, 0xB8, 0xA4, 0xED, 0xA9, 0xA6, 0x13, 0x40, 0xE0,
        0xE4, 0x1E, 0x0B, 0x54, 0x10, 0x0D, 0x14, 0x40, 0xEA, 0x64, 0x85, 0x71, 0xBA, 0x76, 0x73,
        0x14, 0x40, 0xF4, 0xE4, 0xEB, 0xD7, 0x20, 0xDD, 0xD9, 0x14, 0x40, 0xFE, 0x64, 0x52, 0x3E,
        0x87, 0x43, 0x40, 0x15, 0x40, 0x08, 0xE4, 0xB8, 0xA4, 0xED, 0xA9, 0xA6, 0x15, 0x40, 0x12,
        0x64, 0x1F, 0x0B, 0x54, 0x10, 0x0D, 0x16, 0x40, 0x1C, 0xE4, 0x85, 0x71, 0xBA, 0x76, 0x73,
        0x16, 0x40, 0x26, 0x64, 0xEC, 0xD7, 0x20, 0xDD, 0xD9, 0x16, 0x40, 0x30, 0xE4, 0x52, 0x3E,
        0x87, 0x43, 0x40, 0x17, 0x40, 0x3A, 0x64, 0xB9, 0xA4, 0xED, 0xA9, 0xA6, 0x17, 0x40, 0x44,
        0xE4, 0x1F, 0x0B, 0x54, 0x10, 0x0D, 0x18, 0x40, 0x4E, 0x64, 0x86, 0x71, 0xBA, 0x76, 0x73,
        0x18, 0x40, 0x58, 0xE4, 0xEC, 0xD7, 0x20, 0xDD, 0xD9, 0x18, 0x40, 0x62, 0x64, 0x53, 0x3E,
        0x87, 0x43, 0x40, 0x19, 0x40, 0x6C, 0xE4, 0xB9, 0xA4, 0xED, 0xA9, 0xA6, 0x19, 0x40, 0x76,
        0x64, 0x20, 0x0B, 0x54, 0x10, 0x0D, 0x1A, 0x40, 0x80, 0xE4, 0x86, 0x71, 0xBA, 0x76, 0x73,
        0x1A, 0x40, 0x8A, 0x64, 0xED, 0xD7, 0x20, 0xDD, 0xD9, 0x1A, 0x40, 0x94, 0xE4, 0x53, 0x3E,
        0x87, 0x43, 0x40, 0x1B, 0x40, 0x9E, 0x64, 0xBA, 0xA4, 0xED, 0xA9, 0xA6, 0x1B, 0x40, 0xA8,
        0xE4, 0x20, 0x0B, 0x54, 0x10, 0x0D, 0x1C, 0x40, 0xB2, 0x64, 0x87, 0x71, 0xBA, 0x76, 0x73,
        0x1C, 0x40, 0xBC, 0xE4, 0xED, 0xD7, 0x20, 0xDD, 0xD9, 0x1C, 0x40, 0xC6, 0x64, 0x54, 0x3E,
        0x87, 0x43, 0x40, 0x1D, 0x40, 0xD0, 0xE4, 0xBA, 0xA4, 0xED, 0xA9, 0xA6, 0x1D, 0x40, 0xDA,
        0x64, 0x21, 0x0B, 0x54, 0x10, 0x0D, 0x1E, 0x40, 0xE4, 0xE4, 0x87, 0x71, 0xBA, 0x76, 0x73,
        0x1E, 0x40, 0xEE, 0x64, 0xEE, 0xD7, 0x20, 0xDD, 0xD9, 0x1E, 0x40, 0xF8, 0xE4, 0x54, 0x3E,
        0x87, 0x43, 0x40, 0x1F, 0x40, 0x02, 0x64, 0xBB, 0xA4, 0xED, 0xA9, 0xA6, 0x1F, 0x40, 0x0C,
        0xF2, 0x90, 0x05, 0x2A, 0x88, 0x06, 0x20, 0x40, 0x16, 0x32, 0xC4, 0x38, 0x5D, 0xBB, 0x39,
        0x20, 0x40, 0x20, 0x72, 0xF7, 0x6B, 0x90, 0xEE, 0x6C, 0x20, 0x40, 0x2A, 0xB2, 0x2A, 0x9F,
        0xC3, 0x21, 0xA0, 0x20, 0x40, 0x34, 0xF2, 0x5D, 0xD2, 0xF6, 0x54, 0xD3, 0x20, 0x40, 0x3E,
        0x32, 0x91, 0x05, 0x2A, 0x88, 0x06, 0x21, 0x40, 0x48, 0x72, 0xC4, 0x38, 0x5D, 0xBB, 0x39,
        0x21, 0x40, 0x52, 0xB2, 0xF7, 0x6B, 0x90, 0xEE, 0x6C, 0x21, 0x40, 0x5C, 0xF2, 0x2A, 0x9F,
        0xC3, 0x21, 0xA0, 0x21, 0x40, 0x66, 0x32, 0x5E, 0xD2, 0xF6, 0x54, 0xD3, 0x21, 0x40, 0x70,
        0x72, 0x91, 0x05, 0x2A, 0x88, 0x06, 0x22, 0x40, 0x7A, 0xB2, 0xC4, 0x38, 0x5D, 0xBB, 0x39,
        0x22, 0x40, 0x84, 0x32, 0x60, 0x8E, 0x90, 0xEE, 0x6C, 0x22, 0x40, 0x8E, 0x72, 0x14, 0x6F,
        0xD0, 0x21, 0xA0, 0x22, 0x40, 0x98, 0xF2, 0x92, 0x95, 0x10, 0x55, 0xD3, 0x22, 0x40, 0xA2,
        0xD2, 0xDB, 0x01, 0x51, 0x88, 0x06, 0x23, 0x40, 0xAC, 0x32, 0xEF, 0xB3, 0x91, 0xBB, 0x39,
        0x23, 0x40, 0xB6, 0xD2, 0xCC, 0xAB, 0xD2, 0xEE, 0x6C, 0x23, 0x40, 0xC0, 0xD2, 0x74, 0xE9,
        0x13, 0x22, 0xA0, 0x23, 0x40, 0xCA, 0x32, 0xE7, 0x6C, 0x55, 0x55, 0xD3, 0x23, 0x40, 0xD4,
        0xF2, 0x23, 0x36, 0x97, 0x88, 0x06, 0x24, 0x40, 0xDE, 0x12, 0x2B, 0x45, 0xD9, 0xBB, 0x39,
        0x24, 0x40, 0x00, 0x72, 0xFC, 0x99, 0x1B, 0xEF, 0x6C, 0x24, 0x40, 0x0A, 0x52, 0x98, 0x34,
        0x5E, 0x22, 0xA0, 0x24, 0x40, 0x14, 0x92, 0xFE, 0x14, 0xA1, 0x55, 0xD3, 0x24, 0x40, 0x1E,
        0x12, 0x2F, 0x3B, 0xE4, 0x88, 0x06, 0x25, 0x40, 0x28, 0x12, 0x2A, 0xA7, 0x27, 0xBC, 0x39,
        0x25, 0x40, 0x32, 0x52, 0xEF, 0x58, 0x6B, 0xEF, 0x6C, 0x25, 0x40, 0x3C, 0xF2, 0x7E, 0x50,
        0xAF, 0x22, 0xA0, 0x25, 0x40, 0x46, 0xF2, 0xD8, 0x8D, 0xF3, 0x55, 0xD3, 0x25, 0x40, 0x50,
        0x52, 0xFD, 0x10, 0x38, 0x89, 0x06, 0x26, 0x40, 0x5A, 0x12, 0xEC, 0xD9, 0x7C, 0xBC, 0x39,
        0x26, 0x40, 0x64, 0x32, 0xA5, 0xE8, 0xC1, 0xEF, 0x6C, 0x26, 0x40, 0x6E, 0xB2, 0x28, 0x3D,
        0x07, 0x23, 0xA0, 0x26, 0x40, 0x78, 0x92, 0x76, 0xD7, 0x4C, 0x56, 0xD3, 0x26, 0x40, 0x82,
        0xD2, 0x8E, 0xB7, 0x92, 0x89, 0x06, 0x27, 0x40, 0x8C, 0x52, 0x71, 0xDD, 0xD8, 0xBC, 0x39,
        0x27, 0x40, 0x96, 0x52, 0x1E, 0x49, 0x1F, 0xF0, 0x6C, 0x27, 0x40, 0xA0, 0x92, 0x95, 0xFA,
        0x65, 0x23, 0xA0, 0x27, 0x40, 0xAA, 0x52, 0xD7, 0xF1, 0xAC, 0x56, 0xD3, 0x27, 0x40, 0xB4,
        0x52, 0xE3, 0x2E, 0xF4, 0x89, 0x06, 0x28, 0x40, 0xBE, 0xB2, 0xB9, 0xB1, 0x3B, 0xBD, 0x39,
        0x28, 0x40, 0xC8, 0x72, 0x5A, 0x7A, 0x83, 0xF0, 0x6C, 0x28, 0x40, 0xD2, 0x92, 0x9B, 0x70,
        0xCA, 0x23, 0xA0, 0x28, 0x40, 0xDC, 0x52, 0xE8, 0x62, 0xFA, 0x56, 0xD3, 0x28, 0x40, 0xE6,
        0x40, 0x40, 0x40, 0x40,
    ];

    #[test]
    fn dt_read_test() {
        let (pos, dt) = Dtblock::read(&RAW, 0, true);

        assert_eq!(pos, 1140);
    }
}
