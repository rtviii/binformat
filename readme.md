```json
{
            "transaction": {
                "message": {
                    "accountKeys": [
                        "5XLqnSjJBAm1XjAcR76QCn8eB1phEQ3py2VAE2f8pdCQ",
                        "Ax9ujW5B9oqcv59N8m6f1BpTBq2rGeGaBcpKjC5UYsXU",
                        "SysvarC1ock11111111111111111111111111111111",
                        "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH"
                    ],
                    "header": {
                        "numReadonlySignedAccounts"  : 0,
                        "numReadonlyUnsignedAccounts": 2,
                        "numRequiredSignatures"      : 1
                    },
                    "instructions": [


                                                    {
                                                        [byte]
                                                        "programIdIndex": 3,
                                                        [len][]
                                                        "data": "6mJFQCt94hG4CKNYKgVcwqt6CaTGZTpekyvwA3NfDoknSEPiZm6dYb",
                                                        "accounts": [
                                                            0,
                                                            1,
                                                            2
                                                        ],
                                                    }

                    ],
                    "recentBlockhash": "AmHEaeFDhizgkHHv9ZXa8BSZPGf7evJc2UhCPr8KznaM"
                },
                "signatures": [
                    "2yorZs4VQKMrjob7CeaiNTfNSa1zRUboT6oYGg3NsBfPZymaVVBAtnVGVanN8HXt3crC9tCLy6RNoshQTN3DMndi"
                ]
            }
        }

    // --------------------------------------------------------------------------------------
    {
            "transaction": {
                "message": {

                    // [num accounts: 1 byte][num x 32bytes]
                    // "accountKeys": [
                    //     "AmKhZ2k8kq9HwZZr3FMi1UJsyPQLAeg7WCHMfjLow6bp",
                    //     "2DM7z4MxS13BmxHc2aENmDYAiMPvYFnMmDyDDYHfuK7D",
                    //     "SysvarS1otHashes111111111111111111111111111",
                    //     "SysvarC1ock11111111111111111111111111111111",
                    //     "Vote111111111111111111111111111111111111111"
                    // ],

                    // [byte][byte][byte]
                    // "header": {
                    //     "numReadonlySignedAccounts"  : 0,
                    //     "numReadonlyUnsignedAccounts": 3,
                    //     "numRequiredSignatures"      : 1
                    // },

                    // [num accounts: 1 byte][num x 32bytes]
                    // [byte][byte][byte]
                    [num signatures(3rd byte of header)][num x 64 bytes]
                    [ N instructions of different lengths]
                    "signatures": [
                        "279EBedXz4fLvh8iyqiP1CFqVuUh54xi1BaPjYRG6hNomjQM1xB7pEXYZEYy3TRbbfnaWoRXaqgJW4VMrPpgH1Wb"
                    ]

                    [ N instructions of different lengths]
                    "instructions": [
                        {
                            [len:1byte] [len x bytes -----]
                            "accounts": [
                                1,
                                2,
                                3,
                                0
                            ],

                            [len:2bytes][len x bytes--...---]
                            "data": "29z5mr1JoRmJYQ6yp7DsrEbrPynEpLdqB3xAAZFKpw5ZW9xsJKRbWmvBmMnywCGwhSTASU8BsRoFhJTvUXdKCvgrxDh5wM",
                            [byte]

                            "programIdIndex": 4
                        }
                    ],

                    // [32bytes]
                    // "recentBlockhash": "2yUZchZURcMYEGSXHkXD1GnuYGs8KRW66CrDhkBbjLce"
                },
            }
        }
```

# Transaction :

```bash

accounts            ---------------------------- [num accounts: 1 byte][num x 32bytes]
header              ---------------------------- [byte][byte][byte]
signatures          ---------------------------- [n signatures x 32 bytes]
blockhash           ---------------------------- [32bytes]
n instructions of different lengths:
                [
                    prog_index    ----------------------------- [byte       ]
                    account_array ----------------------------- [len :byte  ][len x bytes]
                    data          ----------------------------- [len :2bytes][len x bytes]
                ]
                [
                    prog_index    ----------------------------- [byte       ]
                    account_array ----------------------------- [len :byte  ][len x bytes]
                    data          ----------------------------- [len :2bytes][len x bytes]
                ]
                [
                    prog_index    ----------------------------- [byte       ]
                    account_array ----------------------------- [len :byte  ][len x bytes]
                    data          ----------------------------- [len :2bytes][len x bytes]
                ]


`
```


Lets call _V(x)__ a _variable-length_ array of length up-to _x_. Then _V(1232)_ is byte array that can be anywhere _from 0 to 1232 bytes long_.

# Instruction

```json

                            "programIdIndex": 4
                            "accounts": [1,2,3,0],

                            "data": "29z5mr1JoRmJYQ6yp7DsrEbrPynEpLdqB3xAAZFKpw5ZW9xsJKRbWmvBmMnywCGwhSTASU8BsRoFhJTvUXdKCvgrxDh5wM",


```
Translates to




```rust
INDEX       := [ 0x04 ]                                                # <----- prog_ix
ACCOUNT_IXS := [ 0x04,                                                 # <----- accixs.len
                0x01, 0x02,0x03, 0x00 ]                               # <----- accixs
DATA        := [ 0x00, 0x5e,                                           # <----- ixdata.len
                0x32, 0x39, 0x7A, 0x35, 0x6D, 0x72, 0x31, 0x4A, 0x6F, # .
                0x52, 0x6D, 0x4A, 0x59, 0x51, 0x36, 0x79, 0x70, 0x37, # |
                0x44, 0x73, 0x72, 0x45, 0x62, 0x72, 0x50, 0x79, 0x6E, # | 
                0x45, 0x70, 0x4C, 0x64, 0x71, 0x42, 0x33, 0x78, 0x41, # | 
                0x41, 0x5A, 0x46, 0x4B, 0x70, 0x77, 0x35, 0x5A, 0x57, # |
                0x39, 0x78, 0x73, 0x4A, 0x4B, 0x52, 0x62, 0x57, 0x6D, # |-- ixdata
                0x76, 0x42, 0x6D, 0x4D, 0x6E, 0x79, 0x77, 0x43, 0x47, # | 
                0x77, 0x68, 0x53, 0x54, 0x41, 0x53, 0x55, 0x38, 0x42, # |
                0x73, 0x52, 0x6F, 0x46, 0x68, 0x4A, 0x54, 0x76, 0x55, # |
                0x58, 0x64, 0x4B, 0x43, 0x76, 0x67, 0x72, 0x78, 0x44, # |
                0x68, 0x35, 0x77, 0x4D                                # .
            ]
```
Instruction, schematically:
```rust
PROGRAM_INDEX       : = [1 byte]
ACCOUNT_INDEX_ARRAY : = [acc_len: 1 byte][acc_len ]
DATA                : = [data_len:2 bytes][V(data_len)]
```
If we rearrange things a little bit, we can have all the "size" information at the top and not have to seek "into" and instruction for _data\_len_:
```rust
[1 byte][acc_ix_len: 1 byte][data_len:2 bytes][V( acc_ix )][V(data_len)]
```
Then, for  an instruction:
- first byte is prog index
- then a byte signifying how many account indices there are
- then 2 bytes to signify number of bytes of instruction data

And arithmetic works out to: 

+ Overall size of the instruction is 1 + 1 + 2 + acc_ix_len  + data_len.
+ Account indexes begin at the 5th byte
+ Data array begins at ( 5 + acc_ix_len + 1 )st byte.





Then, an transaction composed of instructions looks something like this:

```rust
ACCOUNT_ADDRESSES : = [ acc_ix_len: 1 byte        ][  V( lenx 32 bytes )     ] 
HEADER            : = [ 3 bytes]
SIGNATURES        : = [ sigs_len:1 byte][V( num x 64 bytes )]
INSTRUCTIONS      : = [ N instructions of different lengths ]

```





- [ ] It'd be sure nice to know the average number of ix/tx and tx/block.

*The entire encoded size of a Solana transaction cannot exceed 1232 bytes.


- Both the instructions and transaction arrays can be sorted by length with the smallest coming in the front to minimize jump lenths in the case of seeks.

- one question is whether to preface every 
