use std::{
    fs::File,
    io::{BufReader, Read, Write}, vec,
};
pub mod transaction;
pub mod utils;
use clap::Parser;
use serde_json::{Value};
use transaction::balances::unpack_pre_post_balances;
use crate::transaction::balances::pack_pre_post_balances;

pub fn parse_tuple(tup: &str) -> Result<(u64, u64), std::string::ParseError> {
    let tup = tup.replace("(", "");
    let tup = tup.replace(")", "");
    let startend: Vec<_> = tup
        .split(",")
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    Ok((startend[0], startend[1]))
}

#[derive(Debug, Parser)]
#[clap(author, version, long_about=None)]
pub struct Args {
    #[clap(takes_value = false, long)]
    decode: Option<String>,

    #[clap(takes_value = false, long)]
    encode: Option<String>,
    // #[clap( long , short='s', value_parser=parse_tuple)]
    // start_end: (u64, u64),
}
fn main() {

    let args = Args::parse();
    let do_decode_path = args.decode;
    let do_encode_path = args.encode;

    if do_encode_path.is_some() {
        let meta = r#"
            {"postBalances": [6743,64,870,280,1],
            "preBalances":   [6743,64,870,280,1]
            }
        "#;

        println!("serialized meta: {}", meta);
        let packed = pack_pre_post_balances(&serde_json::from_str(meta).unwrap());
        let mut f  = File::create(do_encode_path.unwrap()).unwrap();
        f.write_all(packed.as_slice()).unwrap();
    }

    if do_decode_path.is_some() {
        let f                   = File::open(do_decode_path.unwrap()).unwrap();
        let mut reader          = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        unpack_pre_post_balances(2, &buffer);
    }
}
