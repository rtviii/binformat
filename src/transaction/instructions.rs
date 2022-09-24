use serde_json::Value;
pub fn pack_ix(ix: &Value) -> Vec<u8> {
    let ix_prog_index = &ix["programIdIndex"]
        .as_u64()
        .unwrap_or_else(|| panic!("Failed to unpack programIdIndex: {:?}", ix));

    let ix_accounts = &ix["accounts"]
        .as_array()
        .unwrap_or_else(|| panic!("Failed to unpack accounts"));

    let ix_data = &ix["data"]
        .as_str()
        .unwrap_or_else(|| panic!("Failed to unpack data"));

    let acc_indices_u8 = ix_accounts
        .into_iter()
        .map(|a| a.as_u64().unwrap() as u8)
        .collect();
    let accounts_len = ix_accounts.len() as u8;
    let data_bytes = bs58::decode(ix_data).into_vec().unwrap();
    let data_len = (data_bytes.len() as u16).to_le_bytes();

    println!("accounts_len :{}", accounts_len);
    println!("accounts  :{:?}", ix_accounts);
    println!("data  :{:?}", ix_data);
    println!("data len  :{:?}", data_len);

    let result = vec![*ix_prog_index as u8, accounts_len, data_len[0]];
    let yx = [[*ix_prog_index as u8, accounts_len], data_len].concat();

    [yx, acc_indices_u8, data_bytes].concat()
}

pub fn unpack_ix() {}

pub fn pack_inner_ix() {}

pub fn unpack_inner_ix() {}

#[cfg(test)]
mod tests {
    use crate::transaction::instructions::pack_ix;

    #[test]
    fn test_pack_ix() {
        let sample_ix = r#"

                            {
                            "accounts": [
                                    3,
                                    4,
                                    0
                                ],
                                "data": "3DV4nz1KFpQX",
                                "programIdIndex": 21
                            }
        "#;
        println!("serialized meta: {}", sample_ix);
        let packed = pack_ix(&serde_json::from_str(sample_ix).unwrap());

        let s = vec![
            0x15, //<-- prog index
            0x03, // <-- acc len
            0x00, 0x08, // <-- data len,
            0x03, 0x04, 0x00, // <-- acc indices
            0x03, 0x00, 0x27, 0xab, 0xce, 0x01, 0x00, 0x00, 0x00,
        ];

        assert_eq!(packed, s);

        // let shortdata = "3DV4nz1KFpQX";
        // let shortdata_hex = [0x03, 0x00, 0x27, 0xab, 0xce, 0x01, 0x00, 0x00, 0x00];
        // let ixdata__    = "29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h".to_string();
        // let ixdata__hex = [
        //     0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0x2c,
        //     0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x2c, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
        //     0xe3, 0x53, 0x4b, 0x39, 0xb0, 0x34, 0x01, 0xce, 0x1b, 0xd5, 0x67, 0x95, 0x75, 0xa4,
        //     0xb5, 0xa4, 0x75, 0x9a, 0x77, 0x36, 0x71, 0xbe, 0x63, 0xd7, 0xa0, 0x89, 0xae, 0xaf,
        //     0x95, 0xb4, 0x60, 0x6c, 0x01, 0xf6, 0x72, 0x84, 0x62, 0x00, 0x00, 0x00, 0x00,
        // ];

        // println!("hash : {}", ixdata__);
        // println!("hash as slice: {:04X?}", ixdata__.as_bytes());
    }

    #[test]
    fn test_pack_ix_1232_data() {}
}
