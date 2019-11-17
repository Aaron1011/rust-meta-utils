#![feature(rustc_private)]

extern crate serialize as rustc_serialize;

use rustc_serialize::opaque::Decoder;
use rustc_serialize::Decodable;

const METADATA_VERSION: u8 = 5;
const METADATA_HEADER: &[u8; 8] = &[b'r', b'u', b's', b't', 0, 0, 0, METADATA_VERSION];

use byteorder::{BigEndian, ReadBytesExt};


#[derive(RustcEncodable, RustcDecodable, Debug)]
struct CrateRoot {
    name: String
}

struct RawMetadata {
    data: Vec<u8>
}

impl RawMetadata {
    fn decode(&self) -> CrateRoot {
        let offset = METADATA_HEADER.len();
        assert_eq!(&self.data[..METADATA_HEADER.len()], METADATA_HEADER);
        let pos = (&self.data[offset..]).read_u32::<BigEndian>().unwrap();
        println!("Pos: {:?}", pos);

        CrateRoot::decode(&mut Decoder::new(&self.data, pos as usize)).unwrap()
    }
}

fn main() {
    let target = std::env::args().nth(1).unwrap();
    let data = std::fs::read(target).unwrap();
    let raw = RawMetadata { data };
    println!("Metadata: ");
    println!("{:?}", raw.decode());
}
