use std::{fs::File, io::Read};
pub mod block;
pub mod tx;
use serde_json::{to_string_pretty, Value};

// {"message":{"accountKeys":["agsWhfJ5PPGjmzMieWY8BR5o1XRVszUBQ5uFz4CtDiJ","4tZQEGSKs8ttAEGUMpPr99W9K5BbS36oVpVNVgvzQq9j","BXVWezJ9z7NG9vgtEUQTxCJaGHoKhXAmRNsMG2xR98t8","25zsnJFotsH1BCep87Zpw3yts2YY9tdSR4AdTDVdLpou","845sArxPPZVJ7YcWA7uw3EGCUibuZ2am3PqNX48n6g1R","Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],"header":{"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":2,"numRequiredSignatures":2},"instructions":[{"accounts":[],"data":"TnpNdP6pvW3sCP5xL5YjCxu7xiH1vSVXida6eowDU5H9zY4UChqiLceeeDPS","programIdIndex":5},{"accounts":[2,3,1],"data":"3DVaC8fPXTwD","programIdIndex":6},{"accounts":[2,4,1],"data":"3DVaC8fPXTwD","programIdIndex":6}],"recentBlockhash":"HHXreXEndEbp5s8jGH5i6SbihFLmDtrmdTJwk6HfhGPY"},"signatures":["22cYSdKEU9trBs6vtZFoh8cxCyNgEjJXq4kQrqq9ViQBnXu9qG2is8f9nxLA4wmEeaGxpUQ5LcsuSTPetBU3eGmj","54kx7BCQABcSyeaofVumt7nu2MZoo2UAMXcdWiqVkHAm4ZgQhVgYj3QJWdazbp16fJi1giCGATdemQ4Ay29AeqtV"]}
pub fn tx1() -> String {
    let tx = r#"
        {"message":{"accountKeys":[
            "agsWhfJ5PPGjmzMieWY8BR5o1XRVszUBQ5uFz4CtDiJ",
            "4tZQEGSKs8ttAEGUMpPr99W9K5BbS36oVpVNVgvzQq9j",
            "BXVWezJ9z7NG9vgtEUQTxCJaGHoKhXAmRNsMG2xR98t8",
            "25zsnJFotsH1BCep87Zpw3yts2YY9tdSR4AdTDVdLpou",
            "845sArxPPZVJ7YcWA7uw3EGCUibuZ2am3PqNX48n6g1R",
            "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            ],
        "header":
            {"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":2,"numRequiredSignatures":2},
        "instructions":[
                {"accounts":[],"data":"TnpNdP6pvW3sCP5xL5YjCxu7xiH1vSVXida6eowDU5H9zY4UChqiLceeeDPS","programIdIndex":5},
                {"accounts":[2,3,1],"data":"3DVaC8fPXTwD","programIdIndex":6},
                {"accounts":[2,4,1],"data":"3DVaC8fPXTwD","programIdIndex":6}
                ],
        "recentBlockhash":"HHXreXEndEbp5s8jGH5i6SbihFLmDtrmdTJwk6HfhGPY"},
        "signatures":[
                "22cYSdKEU9trBs6vtZFoh8cxCyNgEjJXq4kQrqq9ViQBnXu9qG2is8f9nxLA4wmEeaGxpUQ5LcsuSTPetBU3eGmj",
                "54kx7BCQABcSyeaofVumt7nu2MZoo2UAMXcdWiqVkHAm4ZgQhVgYj3QJWdazbp16fJi1giCGATdemQ4Ay29AeqtV"]}"#;

    tx.to_string()
}

fn main() {
    // open json file and parse it with serde_json
    let mut file =
        File::open("/home/rxz/dev/SolanaBeach/binformat/src/sampledata/block121654072.json")
            .unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let block: Value = serde_json::from_str(&contents).unwrap();
    let onetx = &block["transactions"].as_array().unwrap()[0];
    pack_tx(onetx);
}

pub fn pack_tx(tx: &Value) {
    // println!("Tx : {}", to_string_pretty(&tx).unwrap());
    let meta = &tx["meta"];
    let transaction = &tx["transaction"];

    println!("{}", to_string_pretty(&meta).unwrap());
    pack_pre_post_balances(meta);
}

/// The pre- and post- balances are encoded in the following way:
/// `changed_balances_bitfield` followed by `pre_balances` followed by only those of the `post_balances` that
/// were changed with the lowest bit signifying the lowest index in the balances array.
/// This is to account for the fact that most pre-post fields in meta are the same (no transfer occurred).
///
/// Hence we set the changed accounts to one in the bitfield and assume the post-'s are equal for the zero-ed ones.
/// Ex. if only the 3rd and the 5th of the 5 accounts changed in balances (rest are unchanged), this field looks like `0b00010100` in binary
/// (5 slots extended to a whole byte) aka `0x14` in hex.
/// We list 5 x 8 bytes of pre balances and then only 2 x 8 bytes of post balances right after (for the 3rd and the 5th respectively).
pub fn pack_pre_post_balances(meta: &Value) -> Vec<u8> {
    let pre_array = &meta["preBalances"]
        .as_array()
        .unwrap_or_else(|| panic!("preBalances is not an array"));
    let post_array = &meta["postBalances"]
        .as_array()
        .unwrap_or_else(|| panic!("postBalances is not an array"));

    assert!(
        pre_array.len() == post_array.len(),
        "pre and post balances arrays are not the same length"
    );

    let n_account_octets: usize = pre_array.len() / 8 + 1;
    let encoded = vec![0u8; n_account_octets + 1];

    let mut accumulator: u128 = 0;
    println!("Got accumulator :{}", accumulator);
    for (i, (pre, post)) in pre_array.iter().zip(post_array.iter()).enumerate() {
        println!("Comparing index {}. \tpre: {}, post: {}", i, pre, post);

        // // Given the variation in the number of accounts,
        // // we need to keep track of the byte into which the changed/unchanged will be encoded:
        // // i.e. if there are 8 accounts (8 pairs of pre/posts) there is only 1 byte, but exceeding that
        // // there might be 2,3,4 or more change-encoding bytes for 16,24,32 accounts respectively.
        // let mut position_byte = 0;
        if pre != post {
            println!("Pre != post, {} != {}", pre, post);
            accumulator += 2_u128.pow(i as u32);
        }
    }
    // println!("Got accumulator :{:#08}", accumulator);

    let mut cast = vec![];
    if (n_account_octets) == 1 {
        cast = vec![(accumulator & 0xFF) as u8];
    } else if (n_account_octets) == 2 {
        cast = (accumulator & 0xFFFF).to_le_bytes().to_vec();
    }
    println!("Got cast :{:?}", cast);

    println!("{:?}", vecu8_to_binary_string(&encoded));
    return encoded;
}

pub fn vecu8_to_binary_string(vecu8: &[u8]) -> Vec<String> {
    vecu8.iter().map(|u| format!("{:#08}", u)).collect()
}
