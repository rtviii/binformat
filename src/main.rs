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

// {"message":{"accountKeys":["agsWhfJ5PPGjmzMieWY8BR5o1XRVszUBQ5uFz4CtDiJ","4tZQEGSKs8ttAEGUMpPr99W9K5BbS36oVpVNVgvzQq9j","BXVWezJ9z7NG9vgtEUQTxCJaGHoKhXAmRNsMG2xR98t8","25zsnJFotsH1BCep87Zpw3yts2YY9tdSR4AdTDVdLpou","845sArxPPZVJ7YcWA7uw3EGCUibuZ2am3PqNX48n6g1R","Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],"header":{"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":2,"numRequiredSignatures":2},"instructions":[{"accounts":[],"data":"TnpNdP6pvW3sCP5xL5YjCxu7xiH1vSVXida6eowDU5H9zY4UChqiLceeeDPS","programIdIndex":5},{"accounts":[2,3,1],"data":"3DVaC8fPXTwD","programIdIndex":6},{"accounts":[2,4,1],"data":"3DVaC8fPXTwD","programIdIndex":6}],"recentBlockhash":"HHXreXEndEbp5s8jGH5i6SbihFLmDtrmdTJwk6HfhGPY"},"signatures":["22cYSdKEU9trBs6vtZFoh8cxCyNgEjJXq4kQrqq9ViQBnXu9qG2is8f9nxLA4wmEeaGxpUQ5LcsuSTPetBU3eGmj","54kx7BCQABcSyeaofVumt7nu2MZoo2UAMXcdWiqVkHAm4ZgQhVgYj3QJWdazbp16fJi1giCGATdemQ4Ay29AeqtV"]}
// pub fn tx1() -> String {
//     let tx = r#"
//         {"message":{"accountKeys":[
//             "agsWhfJ5PPGjmzMieWY8BR5o1XRVszUBQ5uFz4CtDiJ",
//             "4tZQEGSKs8ttAEGUMpPr99W9K5BbS36oVpVNVgvzQq9j",
//             "BXVWezJ9z7NG9vgtEUQTxCJaGHoKhXAmRNsMG2xR98t8",
//             "25zsnJFotsH1BCep87Zpw3yts2YY9tdSR4AdTDVdLpou",
//             "845sArxPPZVJ7YcWA7uw3EGCUibuZ2am3PqNX48n6g1R",
//             "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo",
//             "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
//             ],
//         "header":
//             {"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":2,"numRequiredSignatures":2},
//         "instructions":[
//                 {"accounts":[],"data":"TnpNdP6pvW3sCP5xL5YjCxu7xiH1vSVXida6eowDU5H9zY4UChqiLceeeDPS","programIdIndex":5},
//                 {"accounts":[2,3,1],"data":"3DVaC8fPXTwD","programIdIndex":6},
//                 {"accounts":[2,4,1],"data":"3DVaC8fPXTwD","programIdIndex":6}
//                 ],
//         "recentBlockhash":"HHXreXEndEbp5s8jGH5i6SbihFLmDtrmdTJwk6HfhGPY"},
//         "signatures":[
//                 "22cYSdKEU9trBs6vtZFoh8cxCyNgEjJXq4kQrqq9ViQBnXu9qG2is8f9nxLA4wmEeaGxpUQ5LcsuSTPetBU3eGmj",
//                 "54kx7BCQABcSyeaofVumt7nu2MZoo2UAMXcdWiqVkHAm4ZgQhVgYj3QJWdazbp16fJi1giCGATdemQ4Ay29AeqtV"]}"#;

//     tx.to_string()
// }

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
