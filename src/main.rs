use std::{fs::File, io::Read};
pub mod tx;
pub mod block;
use serde_json::{ Value, to_string_pretty };
use solana_sdk::signature::Signature;

// {"message":{"accountKeys":["agsWhfJ5PPGjmzMieWY8BR5o1XRVszUBQ5uFz4CtDiJ","4tZQEGSKs8ttAEGUMpPr99W9K5BbS36oVpVNVgvzQq9j","BXVWezJ9z7NG9vgtEUQTxCJaGHoKhXAmRNsMG2xR98t8","25zsnJFotsH1BCep87Zpw3yts2YY9tdSR4AdTDVdLpou","845sArxPPZVJ7YcWA7uw3EGCUibuZ2am3PqNX48n6g1R","Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],"header":{"numReadonlySignedAccounts":0,"numReadonlyUnsignedAccounts":2,"numRequiredSignatures":2},"instructions":[{"accounts":[],"data":"TnpNdP6pvW3sCP5xL5YjCxu7xiH1vSVXida6eowDU5H9zY4UChqiLceeeDPS","programIdIndex":5},{"accounts":[2,3,1],"data":"3DVaC8fPXTwD","programIdIndex":6},{"accounts":[2,4,1],"data":"3DVaC8fPXTwD","programIdIndex":6}],"recentBlockhash":"HHXreXEndEbp5s8jGH5i6SbihFLmDtrmdTJwk6HfhGPY"},"signatures":["22cYSdKEU9trBs6vtZFoh8cxCyNgEjJXq4kQrqq9ViQBnXu9qG2is8f9nxLA4wmEeaGxpUQ5LcsuSTPetBU3eGmj","54kx7BCQABcSyeaofVumt7nu2MZoo2UAMXcdWiqVkHAm4ZgQhVgYj3QJWdazbp16fJi1giCGATdemQ4Ay29AeqtV"]}
pub fn tx1()->String{

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
    let mut file     = File::open("/home/rxz/dev/SolanaBeach/binformat/src/sampledata/block121654072.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let block: Value = serde_json::from_str(&contents).unwrap();
    let onetx = &block["transactions"]
    .as_array().unwrap()[0];
    pack_tx(onetx);
}

pub fn pack_tx(tx:&Value){
    // println!("Tx : {}", to_string_pretty(&tx).unwrap());
    let meta        = &tx["meta"];
    let transaction = &tx["transaction"];

    println!("{}", to_string_pretty(&meta).unwrap());



}
