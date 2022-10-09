pub mod bin_data;
pub mod body;
pub mod doc_info;
pub mod header;
pub mod paragraph;
pub mod section;
pub mod version;

mod parameter_set;
mod record;
mod utils;

use self::{bin_data::File, body::Body, doc_info::DocInfo, header::Header};

use std::io::{Cursor, Read};

use cfb::CompoundFile;

#[derive(Debug)]
pub struct HWP {
    pub header: Header,
    pub body_texts: Body,
    pub view_texts: Option<Body>,
    pub doc_info: DocInfo,
    pub bin_data: Vec<File>,
}

impl HWP {
    pub fn from_bytes(bytes: &[u8]) -> HWP {
        let cursor = Cursor::new(bytes);
        let mut cfb = CompoundFile::open(cursor).unwrap();

        let header = Header::from_cfb(&mut cfb);

        let doc_info = DocInfo::from_cfb(&mut cfb, &header.version);

        let body_texts = Body::from_cfb(&mut cfb, &header.version);
        let view_texts = if header.flags.distributed {
            Some(Body::from_distributed(&mut cfb, &header.version))
        } else {
            None
        };

        let mut bin_data = vec![];

        for item in &doc_info.bin_data_list {
            let file_name = item.cfb_file_name();
            if file_name.is_some() {
                let name = file_name.unwrap();
                let mut stream = cfb.open_stream(format!("BinData/{}", name)).unwrap();
                let mut data = vec![];
                stream.read_to_end(&mut data).unwrap();
                bin_data.push(File { name, data });
            }
        }

        HWP {
            header,
            doc_info,
            body_texts,
            view_texts,
            bin_data,
        }
    }
}
