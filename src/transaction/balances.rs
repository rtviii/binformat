
use serde_json::{Value};
pub fn rewards(){
    println!("Rewards");
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

    let n_account_octets: usize      = pre_array.len() / 8 + 1;

    let mut change_accumulator: u128 = 0;
    let pre_balances: Vec<u64>       = pre_array.iter().map(|v| v.as_u64().unwrap()).collect();
    let mut post_balances: Vec<u64>  = vec![];

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

    [pre_balances, post_balances]
        .concat()
        .into_iter()
        .for_each(|v| {
            change_bitfield.extend_from_slice(&v.to_le_bytes());
        });

    return change_bitfield;
}

// -------------------------- Unpack

/// Needing the `n_accounts`arg here just because it will be present in the top level tx, but we'd like to test this separately.
pub fn unpack_pre_post_balances(n_accounts: usize, buff: &[u8]) -> (Vec<u64>, Vec<u64>) {
    let n_account_octets: usize = n_accounts as usize / 8 + 1;

    
    let change_bitfield_u128 = u128::from_le_bytes([
        &buff[0..n_account_octets], 
        &vec![0;16-n_account_octets] ].concat().try_into().expect("Failed to build u128")
    );


    let pre_: &[u8]              = &buff[n_account_octets..n_account_octets + 8 * n_accounts as usize];
    let post_: &[u8]             = &buff[n_account_octets + 8 * n_accounts as usize..];

    let mut pre_balances:Vec<u64>  = vec![];
    let mut post_balances:Vec<u64> = vec![];

    pre_.chunks(8).into_iter().for_each(|c| {
            pre_balances.push(u64::from_le_bytes(c.try_into().expect("Incorrect length of array. Failed to build u64.")));
    });
    let mut post_iter = post_.chunks(8).into_iter();
    let mut countdown = 0;

    while countdown != n_accounts {
        
        if ( change_bitfield_u128 >> countdown ) & 1 == 0 {
            post_balances.push(pre_balances[countdown]);
        }else{
            post_balances.push(u64::from_le_bytes(post_iter.next().expect("Insufficient post balances in encoded array.").try_into().unwrap()));
        }

        countdown += 1;
    }

    return (pre_balances, post_balances);
}
#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::{
        fs::File,
        io::{BufReader, Read, Write},
    };

    use serde_json::Value;

    use crate::{pack_pre_post_balances, unpack_pre_post_balances};

    #[test]
    fn pre_post_nochange() {
        let balances = b"
            {
            \"preBalances\": [1,2,3,4,5],
            \"postBalances\" : [1,2,3,4,5]
            }";
        let balances: &Value = &serde_json::from_slice(balances).unwrap();

        let mut head = vec![0u8];
        let balance_vals: [u64; 5] = [1, 2, 3, 4, 5];
        let _ = balance_vals
            .iter()
            .for_each(|v| head.extend_from_slice(&v.to_le_bytes()));

        assert_eq!(pack_pre_post_balances(balances), head);
    }

    #[test]
    fn pre_post_allchange() {
        let balances = b"
            {
            \"preBalances\": [1,2,3,4,5],
            \"postBalances\" : [2,3,4,5,6]  
            }";
        let balances: &Value = &serde_json::from_slice(balances).unwrap();

        let mut head = vec![(1 + 2 + 4 + 8 + 16) as u8];
        let balance_vals: [u64; 10] = [1, 2, 3, 4, 5, 2, 3, 4, 5, 6];
        let _ = balance_vals
            .iter()
            .for_each(|v| head.extend_from_slice(&(*v).to_le_bytes()));
        assert_eq!(pack_pre_post_balances(balances), head);
    }

    #[test]
    #[should_panic]
    fn pre_post_empty_balances() {
        println!("This test should panic.");
        // No accountless transactions allowed
        let balances = b"
            {
            \"pretBalances\": [],
            \"postBalances\" : []  
            }";
        let balances: &Value = &serde_json::from_slice(balances).unwrap();

        let mut head = vec![0_u8];
        let balance_vals: [u64; 0] = [];
        let _ = balance_vals
            .iter()
            .for_each(|v| head.extend_from_slice(&v.to_le_bytes()));
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

        let mut head = vec![0_u8, 0_u8, 0_u8];
        let balance_vals: [u64; 17] = [
            10000000004,
            20000000003,
            300000000,
            4000000,
            5,
            60000000004,
            70000000003,
            800000000,
            9000000,
            10,
            11000000004,
            12000000003,
            130000000,
            1400000,
            15,
            1600000,
            17,
        ];
        let _ = balance_vals
            .iter()
            .for_each(|v| head.extend_from_slice(&v.to_le_bytes()));
        assert_eq!(pack_pre_post_balances(balances), head);
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
        let mut head: Vec<u8> =
            ((2_i32.pow(4) + 2_i32.pow(9) + 2_i32.pow(16)) as u128).to_le_bytes()[0..3].to_vec();
        let balance_vals: [u64; 20] = [
            10000000004,
            20000000003,
            300000000,
            4000000,
            5,
            60000000004,
            70000000003,
            800000000,
            9000000,
            10,
            11000000004,
            12000000003,
            130000000,
            1400000,
            15,
            1600000,
            17,
            5555,
            10222,
            17000,
        ];
        let _ = balance_vals
            .iter()
            .for_each(|v| head.extend_from_slice(&(*v).to_le_bytes()));
        assert_eq!(pack_pre_post_balances(balances), head);
    }

    #[test]
    fn pre_post_decode_same() {
        let dummypath = "312415412151251.beach";
        let pre     = vec![1, 2, 3, 4, 5];
        let post    = vec![1, 2, 3, 4, 5];
        let len_pre = pre.len();

        let meta = r#"
            {
            "preBalances" : [1, 2, 3, 4, 5],
            "postBalances": [1, 2, 3, 4, 5]
            }
        "#;

        let packed = pack_pre_post_balances(&serde_json::from_str(meta).unwrap());
        let mut f = File::create(dummypath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(dummypath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let (pre_result, post_result) = unpack_pre_post_balances(len_pre, &buffer);

        assert_eq!((pre_result, post_result), (pre, post));
        remove_file(dummypath).unwrap();
    }

    #[test]
    fn pre_post_decode_changed() {
        let dummypath = "312415412151252.beach";
        let pre     = vec![1, 2, 3, 4, 5,0,0,0];
        let post    = vec![1, 2, 3, 8, 5,0,0,200];
        let len_pre = pre.len();

        let meta = r#"
            {
            "preBalances" : [1, 2, 3, 4, 5,0,0,0],
            "postBalances": [1, 2, 3, 8, 5,0,0,200]
            }
        "#;

        let packed = pack_pre_post_balances(&serde_json::from_str(meta).unwrap());
        let mut f = File::create(dummypath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(dummypath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let (pre_result, post_result) = unpack_pre_post_balances(len_pre, &buffer);

        assert_eq!((pre_result, post_result), (pre, post));
        remove_file(dummypath).unwrap();
    }

    #[test]
    fn pre_post_decode_multioctet() {
        let dummypath = "312415412151253.beach";
        let pre     = vec![1, 2, 3, 4, 5,0,0,0,1, 2, 3, 8, 5,0,0,200];
        let post    = vec![1, 2, 3, 8, 15,0,0,200,1, 2, 14124124, 8, 5,12,0,200];
        let len_pre = pre.len();

        let meta = r#"
            {
            "preBalances" : [1, 2, 3, 4, 5,0,0,0,1, 2, 3, 8, 5,0,0,200],
            "postBalances": [1, 2, 3, 8, 15,0,0,200,1, 2, 14124124, 8, 5,12,0,200]
            }
        "#;

        let packed = pack_pre_post_balances(&serde_json::from_str(meta).unwrap());
        let mut f = File::create(dummypath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(dummypath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let (pre_result, post_result) = unpack_pre_post_balances(len_pre, &buffer);

        assert_eq!((pre_result, post_result), (pre, post));
        remove_file(dummypath).unwrap();
    }
}