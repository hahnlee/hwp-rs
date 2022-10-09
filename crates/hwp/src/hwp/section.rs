use crate::hwp::{record::reader::read_records, utils::random::SRand};

use super::{
    header::Header,
    paragraph::Paragraph,
    record::{reader::RecordReader, tags::DocInfoRecord},
    utils::crypto::decrypt_aes_128_ecb,
    version::Version,
};

use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::DeflateDecoder;

#[derive(Debug)]
pub struct Section {
    pub paragraphs: Vec<Paragraph>,
}

impl Section {
    pub fn from_reader<T: Read>(reader: &mut T, version: &Version) -> Section {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).unwrap();

        let records = read_records(data);

        let paragraphs = records
            .into_iter()
            .map(|mut record| Paragraph::from_record(&mut record, version))
            .collect();

        Section { paragraphs }
    }

    pub fn from_stream<T: Read>(stream: &mut T, header: &Header) -> Section {
        if header.flags.compressed {
            let mut data = DeflateDecoder::new(stream);
            return Section::from_reader(&mut data, &header.version);
        }

        return Section::from_reader(stream, &header.version);
    }

    pub fn from_distributed<T: Read>(stream: &mut T, header: &Header) -> Section {
        let (tag_id, _, size, mut reader) = stream.read_record::<LittleEndian>().unwrap();

        if tag_id != DocInfoRecord::HWPTAG_DISTRIBUTE_DOC_DATA as u32 || size != 256 {
            // TODO: (@hahnlee) 옵셔널
            panic!("올바르지 않은 정보");
        }

        let mut data = [0u8; 256];
        reader.read_exact(&mut data).unwrap();

        let mut seed_cursor = Cursor::new(&data[0..4]);
        let seed = seed_cursor.read_u32::<LittleEndian>().unwrap();
        let mut rand = SRand::new(seed);

        let mut random_numbers = [0u8; 256];
        let mut i = 0;
        loop {
            if i == 256 {
                break;
            }
            let fill = rand.rand() & 0xFF;
            let times = (rand.rand() & 0x0F) + 1;

            for _ in 0..times {
                if i == 256 {
                    break;
                }

                random_numbers[i] = fill as u8;
                i += 1;
            }
        }

        let offset = ((seed & 0xF) + 4) as usize;

        let mut out = [0u8; 256];
        for i in 0..256 {
            out[i] = data[i] ^ random_numbers[i];
        }

        let hash_code = &out[offset..offset + 80];
        let decryption_key = &hash_code[0..16];

        let mut encrypted: Vec<u8> = Vec::new();
        stream.read_to_end(&mut encrypted).unwrap();

        let decrypted = decrypt_aes_128_ecb(&decryption_key, &encrypted);

        let mut cursor = Cursor::new(decrypted);

        if header.flags.compressed {
            let mut decoded = DeflateDecoder::new(cursor);
            return Section::from_reader(&mut decoded, &header.version);
        }

        return Section::from_reader(&mut cursor, &header.version);
    }
}
