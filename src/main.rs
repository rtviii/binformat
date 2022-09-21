use std::{fs::File, io::{Read, Write, BufReader}};
pub mod block;
pub mod tx;
use serde_json::{to_string_pretty, Value};
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;
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
    // let mut file =
    //     File::open("/home/rxz/dev/SolanaBeach/binformat/src/sampledata/block121654072.json").unwrap();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let block: Value = serde_json::from_str(&contents).unwrap();
    // let onetx = &block["transactions"].as_array().unwrap()[0];
    // pack_tx(onetx);


    unpack_pre_post_balances(2, &[]);
}

pub fn pack_tx(tx: &Value) {
    // println!("Tx : {}", to_string_pretty(&tx).unwrap());
    let meta = &tx["meta"];
    let transaction = &tx["transaction"];

    println!("{}", to_string_pretty(&meta).unwrap());
    let packed = pack_pre_post_balances(meta);
    let mut f = File::create("sample_encoded.sbbf").unwrap();
        f.write_all(packed.as_slice()).unwrap();
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

    let mut change_accumulator: u128 = 0;
    let pre_balances: Vec<u64> = pre_array.iter().map(|v| v.as_u64().unwrap()).collect();
    let mut post_balances: Vec<u64> = vec![];
    // println!("Got accumulator :{}", accumulator);

    for (i, (pre, post)) in pre_array.iter().zip(post_array.iter()).enumerate() {

        if pre != post {
            change_accumulator += 2_u128.pow(i as u32);
            post_balances.push(post.as_u64().unwrap());
        }
    }

    let mut change_bitfield: Vec<u8> = change_accumulator
        .to_le_bytes()
        .into_iter()
        .take(n_account_octets)
        .collect();

    [pre_balances,post_balances ].concat()
        .into_iter()
        .for_each(|v| {
            change_bitfield.extend_from_slice(&v.to_le_bytes());
        });

    return change_bitfield;
}

pub fn vecu8_to_binary_string(vecu8: &[u8]) -> Vec<String> {
    vecu8.iter().map(|u| format!("{:#010b}", u)).collect()
}


// -------------------------- Unpack

pub fn unpack_pre_post_balances(n_accounts:u8,buff: &[ u8 ])-> (Vec<u64>, Vec<u64>) {

    let f                  = File::open("sample_encoded.beach").unwrap();
    let mut reader         = BufReader::new(f);
    let mut buffer:Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    for value in buffer {
        println!("BYTE: {:#04x}", value);
    }
    return ( vec![0],vec![0] )
}



#[cfg(test)]
mod tests {
    use std::io::Read;

    use serde_json::Value;

    use crate::pack_pre_post_balances;

    #[test]
    fn pre_post_nochange() {
        let balances = b"
            {
            \"preBalances\": [1,2,3,4,5],
            \"postBalances\" : [1,2,3,4,5]
            }";
            let balances: &Value = &serde_json::from_slice(balances).unwrap();

            let mut head             = vec![0u8];
            let balance_vals:[u64;5] = [1,2,3,4,5];
            let _                    = balance_vals.iter().for_each(|v| head.extend_from_slice(&v.to_le_bytes()));
            assert_eq!(pack_pre_post_balances(balances),head);
    }

    #[test]
    fn pre_post_allchange() {
        let balances = b"
            {
            \"preBalances\": [1,2,3,4,5],
            \"postBalances\" : [2,3,4,5,6]  
            }";
            let balances: &Value = &serde_json::from_slice(balances).unwrap();

            let mut head             = vec![( 1+2+4+8+16 ) as u8];
            let balance_vals:[u64;10] = [1,2,3,4,5,2,3,4,5,6];
            let _                    = balance_vals.iter().for_each(|v| head.extend_from_slice(&(*v).to_le_bytes()));
            assert_eq!(pack_pre_post_balances(balances),head);
    }

    #[test]
    #[should_panic]
    fn pre_post_empty_balances() { 

        // No accountless transactions allowed
        let balances = b"
            {
            \"pretBalances\": [],
            \"postBalances\" : []  
            }";
            let balances: &Value = &serde_json::from_slice(balances).unwrap();

            let mut head             = vec![0_u8];
            let balance_vals:[u64;0] = [];
            let _                    = balance_vals.iter().for_each(|v| head.extend_from_slice(&v.to_le_bytes()));
            pack_pre_post_balances(balances);
    
        }

    #[test]
    fn pre_post_multioctet_nochange() { 
        let balances = b"
            {
            \"preBalances\": [
                    10000000004,20000000003,300000000,4000000,5,60000000004,70000000003,800000000,9000000,10,11000000004,12000000003,130000000,1400000,15,1600000,17
            ],
            \"postBalances\" : [
                    10000000004,20000000003,300000000,4000000,5,60000000004,70000000003,800000000,9000000,10,11000000004,12000000003,130000000,1400000,15,1600000,17
            ]  
            }";
            let balances: &Value = &serde_json::from_slice(balances).unwrap();

            let mut head             = vec![0_u8, 0_u8, 0_u8];
            let balance_vals:[u64;17] = [
                    10000000004,20000000003,300000000,4000000,5,60000000004,70000000003,800000000,9000000,10,11000000004,12000000003,130000000,1400000,15,1600000,17,
            ];
            let _                    = balance_vals.iter().for_each(|v| head.extend_from_slice(&v.to_le_bytes()));
            assert_eq!(pack_pre_post_balances(balances),head);
    }

    #[test]
    fn pre_post_multioctet_changed() { 
        let balances = b"
            {
            \"preBalances\": [

                    10000000004,20000000003,300000000,4000000,5,60000000004,70000000003,800000000,9000000,10,11000000004,12000000003,130000000,1400000,15,1600000,17
            ],
            \"postBalances\" : [
                    10000000004,20000000003,300000000,4000000,5555,60000000004,70000000003,800000000,9000000,10222,11000000004,12000000003,130000000,1400000,15,1600000,17000
            ]  
            }";
            let balances: &Value = &serde_json::from_slice(balances).unwrap();
            // println!("Chnaged :{:#0130b}",( (2_i32.pow(4) + 2_i32.pow(9)  + 2_i32.pow(16)) as u128 ));
            // println!("into bytes :{:#?}",( (2_i32.pow(4) + 2_i32.pow(9)  + 2_i32.pow(16)) as u128 ).to_le_bytes());
            // println!("into bytes :{:#?}",( (2_i32.pow(4) + 2_i32.pow(9)  + 2_i32.pow(16)) as u128 ).to_le_bytes());
            let mut head:Vec<u8> = ( (2_i32.pow(4) + 2_i32.pow(9)  + 2_i32.pow(16)) as u128 ).to_le_bytes()[0..3].to_vec();
            let balance_vals:[u64;20] = [
                    10000000004,20000000003,300000000,4000000,5,60000000004,70000000003,800000000,9000000,10,11000000004,12000000003,130000000,1400000,15,1600000,17,
                    5555, 10222,17000

            ];
            let _= balance_vals.iter().for_each(|v| { head.extend_from_slice(&(*v).to_le_bytes()) });
            assert_eq!(pack_pre_post_balances(balances),head);
    }
}



