use serde_json::Value;
use solana_sdk::instruction::CompiledInstruction;
use solana_transaction_status::InnerInstructions;
use std::{
    slice::SliceIndex,
    sync::{Arc, Mutex},
    vec,
};
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
    let result = vec![*ix_prog_index as u8, accounts_len, data_len[0]];
    let yx = [[*ix_prog_index as u8, accounts_len], data_len].concat();

    [yx, acc_indices_u8, data_bytes].concat()
}

pub fn unpack_ix(buffer: &[u8]) -> (CompiledInstruction, usize) {
    let prog_index: u8 = buffer[0];
    let acc_len = buffer[1];
    let data_len = u16::from_le_bytes([buffer[2], buffer[3]]);
    let acc_indices: Vec<u8> = (4..4 + acc_len).map(|i| buffer[i as usize]).collect();
    let data: Vec<u8> =
        Vec::from(&buffer[(4 + acc_len as usize)..((4 + acc_len as u16 + data_len) as usize)]);
    (
        CompiledInstruction {
            accounts: acc_indices,
            data,
            program_id_index: prog_index,
        },
        (4 + acc_len as u16 + data_len) as usize,
    )
}

///`total_inner_ix_len` signifies how many bytes long is the size of this inner instruction: `CompiledInstruction`s (sum of individual lengths of each instructions) + 1 for `inner_ix_index`  + 2 bytes for itself is is enough information to be able to skip to the end of the given inner_instruction:
///`jmp(total_inner_ix_len) is where the next inner instruction begins in the higher-level trnasaction structure.
pub fn pack_inner_ix(v: &Value) -> Vec<u8> {
    let ix_inner_index = v["index"]
        .as_u64()
        .unwrap_or_else(|| panic!("Failed to unpack index: {:?}", &v));

    let ix_inner_instructions = v["instructions"]
        .as_array()
        .unwrap_or_else(|| panic!("failed to unpack accounts {:?}", &v));
    let mut index_and_data = vec![ix_inner_index as u8];
    for ix_inner_ix in ix_inner_instructions {
        index_and_data.extend_from_slice(&pack_ix(ix_inner_ix))
    }

    let mut r = (index_and_data.len() as u16 + 2).to_le_bytes().to_vec();
    r.append(&mut index_and_data);
    r
}

pub trait BeachOps {
    fn extract_instruction(&self) -> CompiledInstruction;
}

/// This is to stand in for a general buffer with SBBF operation handles defined on it.
/// Once a component has been extracted from the buffer, advance the offset.
/// This is a general harness for buffers and should be differentiated further.
pub struct BufferWindow<'life> {
    pub buffer: &'life [u8],
    // how many bytes have been read so far.
    offset: Arc<Mutex<usize>>,
}

impl BeachOps for BufferWindow<'_> {
    fn extract_instruction(&self) -> CompiledInstruction {
        let (ix, readlen) =
            unpack_ix(&self.buffer[*self.offset.lock().expect("Failed to acquire lock")..]);
        *self
            .offset
            .lock()
            .expect("Could not acquire lock on the Beach buffer.") += readlen;
        ix
    }
}

pub fn unpack_inner_ix(buffer: &[u8]) -> InnerInstructions {

    let total_length          = u16::from_le_bytes([buffer[0], buffer[1]]);
    println!("Got total length :{}", total_length);
    let index                 = buffer[2];
    let instruction_raw_bytes = &buffer[3..];

    let bw = BufferWindow {
        buffer: instruction_raw_bytes,
        offset: Arc::new(Mutex::new(0)),
    };

    let mut inner_ix_ixs = vec![];
    if let Ok(offset) = bw.offset.try_lock() {
        // while the offset of the buffer is still below the sum lengths of compiled instructions 
        // there are instructions left in this inner_ix
        while  *offset < ( total_length.checked_sub(2).expect("Total length less than 2. Soemthing went terribly wrong") ) as usize{
            let ix = bw.extract_instruction();
            inner_ix_ixs.push(ix);
        }
    } else {
        panic!("Failed to acquire buffer lock.")
    };

    InnerInstructions {
        index,
        instructions: inner_ix_ixs
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufReader, Read, Write},
        sync::{Arc, Mutex},
    };

    use solana_sdk::instruction::CompiledInstruction;
    use solana_transaction_status::InnerInstructions;

    use crate::transaction::instructions::{
        pack_inner_ix, pack_ix, unpack_inner_ix, unpack_ix, BeachOps, BufferWindow,
    };

    #[test]
    fn test_unpack_ix_via_buffer() {
        let sample_ix2 = r#"


                            {
                            "accounts": [
                                    1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16
                                ],
                                "data": "29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h",
                                "programIdIndex": 42
                            }
        "#;

        let inner_ixpath = "ix_beachbuffer_test12359132.beach";

        let packed = pack_ix(&serde_json::from_str(sample_ix2).unwrap());
        let mut f = File::create(inner_ixpath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(inner_ixpath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let mut bw = BufferWindow {
            buffer: &buffer,
            offset: Arc::new(Mutex::new(0)),
        };

        let ix = bw.extract_instruction();
        println!("Bw offset is now {:?}", bw.offset);

        assert_eq!(ix, CompiledInstruction{
            accounts: vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],
            data: bs58::decode("29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h").into_vec().unwrap(),
            program_id_index: 42
        });

        std::fs::remove_file(inner_ixpath).unwrap();
    }

    // #[test]
    // fn test_pack_ix_1232_data() {
    //     todo!();
    // }

    #[test]
    fn test_inner_ix_pack_simple() {
        let sample_inner_ix_raw = r#"
        {
                        "index": 0,
                        "instructions": [
                            {
                                "accounts": [
                                    7,
                                    5,
                                    0,
                                    1,
                                    6,
                                    4
                                ],
                                "data": "5QCjNa7",
                                "programIdIndex": 11
                            },
                            {
                                "accounts": [
                                    7,
                                    5,
                                    0,
                                    8,
                                    1,
                                    6,
                                    4,
                                    2,
                                    5
                                ],
                                "data": "3MAbPtimd3JYiSTVY2HAvv6uzrEKgbGFp65qYZMJrk3bHR6UXW68nV7RweeGAVgb7y",
                                "programIdIndex": 11
                            },
                            {
                                "accounts": [
                                    7,
                                    5,
                                    0,
                                    8,
                                    1,
                                    6,
                                    4,
                                    2,
                                    5
                                ],
                                "data": "3MAbPtv9eiPcs7CcyueLdUoXwQYS9RTNaCQGuERC1JLQhvxk3rRU327nDEViyUzwxo",
                                "programIdIndex": 11
                            }
                        ]
                    }"#;

        let inner_ixpath = "inner_ix_test124142.beach";
        let packed = pack_inner_ix(&serde_json::from_str(sample_inner_ix_raw).unwrap());
        let mut f = File::create(inner_ixpath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(inner_ixpath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();
        println!("Returning r :{:#04X?}", &buffer);
        let inner_ix = unpack_inner_ix(&buffer);

        assert!(inner_ix.eq(&InnerInstructions {
            index: 0,
            instructions: vec![
                CompiledInstruction {
                    accounts: vec![7, 5, 0, 1, 6, 4],
                    data: bs58::decode("5QCjNa7").into_vec().unwrap(),
                    program_id_index: 11
                },
                CompiledInstruction {
                    accounts: vec![7, 5, 0, 8, 1, 6, 4, 2, 5],
                    data: bs58::decode(
                        "3MAbPtimd3JYiSTVY2HAvv6uzrEKgbGFp65qYZMJrk3bHR6UXW68nV7RweeGAVgb7y"
                    )
                    .into_vec()
                    .unwrap(),
                    program_id_index: 11
                },
                CompiledInstruction {
                    accounts: vec![7, 5, 0, 8, 1, 6, 4, 2, 5],
                    program_id_index: 11,
                    data: bs58::decode(
                        "3MAbPtv9eiPcs7CcyueLdUoXwQYS9RTNaCQGuERC1JLQhvxk3rRU327nDEViyUzwxo"
                    )
                    .into_vec()
                    .unwrap(),
                },
            ]
        }));
        std::fs::remove_file(inner_ixpath).unwrap();
    }

    #[test]
    fn test_pack_unpack_ix() {
        let sample_ix2 = r#"


                            {
                            "accounts": [
                                    1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16
                                ],
                                "data": "29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h",
                                "programIdIndex": 42
                            }
        "#;

        let inner_ixpath = "ix_test1235912.beach";

        let packed = pack_ix(&serde_json::from_str(sample_ix2).unwrap());
        let mut f = File::create(inner_ixpath).unwrap();
        f.write_all(packed.as_slice()).unwrap();

        let f = File::open(inner_ixpath).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_to_end(&mut buffer).unwrap();

        let (ix, _) = unpack_ix(&buffer);

        assert_eq!(ix, CompiledInstruction{
            accounts: vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],
            data: bs58::decode("29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h").into_vec().unwrap(),
            program_id_index: 42
        });

        std::fs::remove_file(inner_ixpath).unwrap();
    }

    #[test]
    fn test_pack_ix_longer() {
        let sample_ix2 = r#"


                            {
                            "accounts": [
                                    1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16
                                ],
                                "data": "29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h",
                                "programIdIndex": 42
                            }
        "#;

        let s2 = vec![
            0x2A, //<-- prog index
            0x10, // <-- acc len
            0x45, 0x00, // <-- data len(69 = 64+5)
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, // <-- acc indices
            0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfa, 0x2c,
            0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0xfb, 0x2c, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
            0xe3, 0x53, 0x4b, 0x39, 0xb0, 0x34, 0x01, 0xce, 0x1b, 0xd5, 0x67, 0x95, 0x75, 0xa4,
            0xb5, 0xa4, 0x75, 0x9a, 0x77, 0x36, 0x71, 0xbe, 0x63, 0xd7, 0xa0, 0x89, 0xae, 0xaf,
            0x95, 0xb4, 0x60, 0x6c, 0x01, 0xf6, 0x72, 0x84, 0x62, 0x00, 0x00, 0x00, 0x00,
        ];

        let packed = pack_ix(&serde_json::from_str(sample_ix2).unwrap());
        assert_eq!(packed, s2);
    }

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
        let packed = pack_ix(&serde_json::from_str(sample_ix).unwrap());

        let s = vec![
            0x15, //<-- prog index
            0x03, // <-- acc len
            0x09, 0x00, // <-- data len,
            0x03, 0x04, 0x00, // <-- acc indices
            0x03, 0x00, 0x27, 0xab, 0xce, 0x01, 0x00, 0x00, 0x00,
        ];

        assert_eq!(packed, s);

        // let shortdata = "3DV4nz1KFpQX";
        // let shortdata_hex = [0x03, 0x00, 0x27, 0xab, 0xce, 0x01, 0x00, 0x00, 0x00];
    }
    // --------- (((((((())))))))

    // ---------------------------------------- INNER IXs
}
