use crate::utils;


#[derive(Debug, Clone, PartialEq)]
pub struct Ceblock {
    pub block_type: [u8; 2],
    pub block_size: u16,
    pub extension_type: u16,
    pub additional: Vec<u8>,
}

impl Ceblock {
    #[allow(dead_code)]
    pub fn write() {}
    #[allow(dead_code)]
    pub fn read(stream: &[u8], little_endian: bool) -> (Ceblock, usize) {
        let mut position = 0;
        let block_type: [u8; 2] = stream[position..position + 2].try_into().expect("msg");
        position += block_type.len();
        let block_size: u16 = utils::read(stream, little_endian, &mut position);
        let extension_type: u16 = utils::read(stream, little_endian, &mut position);

        let additional = stream[position..block_size as usize].to_vec();

        (
            Ceblock {
                block_type,
                block_size,
                extension_type,
                additional,
            },
            position,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let ce_data = [
            0x43, 0x45, 0x80, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x68,
            0x61, 0x6E, 0x6E, 0x65, 0x6C, 0x20, 0x69, 0x6E, 0x73, 0x65, 0x72, 0x74, 0x65, 0x64,
            0x20, 0x62, 0x79, 0x20, 0x50, 0x79, 0x74, 0x68, 0x6F, 0x6E, 0x20, 0x53, 0x63, 0x72,
            0x69, 0x70, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x43, 0x4E, 0xE4, 0x00, 0xA6, 0xE3, 0x10, 0x00, 0x80, 0xE0, 0x10, 0x00,
            0xAE, 0xE0, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x74, 0x69, 0x6D, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let (ce_block, _position) = Ceblock::read(&ce_data, true);

        // assert_eq!(position, 0);
        assert_eq!(ce_block.block_size, 128);
        assert_eq!(ce_block.extension_type, 2);
    }

    #[test]
    fn write() {}
}