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
                    
                    [ N instructions of different lengths]
                    "instructions": [
                        {
                            [len:1byte][len x bytes -----]
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
                [n signatures]
                "signatures": [
                    "279EBedXz4fLvh8iyqiP1CFqVuUh54xi1BaPjYRG6hNomjQM1xB7pEXYZEYy3TRbbfnaWoRXaqgJW4VMrPpgH1Wb"
                ]
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


```



- [ ] It'd be sure nice to know the average number of ix/tx and tx/block.

*The entire encoded size of a Solana transaction cannot exceed 1232 bytes. 

Instruction:
- accounts indexes. variable number of elements( under 256 len? ). elements are unsigned integers 
- data of variable length. bytes for length followed by the data itself. given that max size is 1232 < 2^16 could just make the length two bytes.
- progindex. 


Both the instructions and transaction arrays can be sorted by length with the smallest coming in the front to minimize jump lenths in the case of seeks.

