const METADATA_VERSION: u8 = 5;
const METADATA_HEADER: &[u8; 8] = &[b'r', b'u', b's', b't', 0, 0, 0, METADATA_VERSION];

use byteorder::{BigEndian, ReadBytesExt};

struct CrateRoot {}

struct RawMetadata {
    data: Vec<u8>
}

impl RawMetadata {
    fn decode(&self) -> CrateRoot {
        let offset = METADATA_HEADER.len();
        assert_eq!(&self.data[..METADATA_HEADER.len()], METADATA_HEADER);
        let pos = (&self.data[offset..]).read_u32::<BigEndian>();
        println!("Pos: {:?}", pos);
        CrateRoot {}
    }
}

fn main() {
    let target = std::env::args().nth(1).unwrap();
    let data = std::fs::read(target).unwrap();
    let raw = RawMetadata { data };
    raw.decode();
}
