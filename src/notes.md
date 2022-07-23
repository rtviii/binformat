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
                        "numReadonlySignedAccounts": 0,
                        "numReadonlyUnsignedAccounts": 2,
                        "numRequiredSignatures": 1
                    },
                    "instructions": [
                        {
                            "accounts": [
                                0,
                                1,
                                2
                            ],
                            "data": "6mJFQCt94hG4CKNYKgVcwqt6CaTGZTpekyvwA3NfDoknSEPiZm6dYb",
                            "programIdIndex": 3
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
            "meta": {
                "err": null,
                "fee": 5000,
                "innerInstructions": [],
                "logMessages": [
                    "Program Vote111111111111111111111111111111111111111 invoke [1]",
                    "Program Vote111111111111111111111111111111111111111 success"
                ],
                "postBalances": [
                    2856625694,
                    26858640,
                    143487360,
                    1169280,
                    1
                ],
                "postTokenBalances": [],
                "preBalances": [
                    2856630694,
                    26858640,
                    143487360,
                    1169280,
                    1
                ],
                "preTokenBalances": [],
                "rewards": [],
                "status": {
                    "Ok": null
                }
            },
            "transaction": {
                "message": {
                    "accountKeys": [
                        "AmKhZ2k8kq9HwZZr3FMi1UJsyPQLAeg7WCHMfjLow6bp",
                        "2DM7z4MxS13BmxHc2aENmDYAiMPvYFnMmDyDDYHfuK7D",
                        "SysvarS1otHashes111111111111111111111111111",
                        "SysvarC1ock11111111111111111111111111111111",
                        "Vote111111111111111111111111111111111111111"
                    ],
                    "header": {
                        "numReadonlySignedAccounts": 0,
                        "numReadonlyUnsignedAccounts": 3,
                        "numRequiredSignatures": 1
                    },
                    "instructions": [
                        {
                            "accounts": [
                                1,
                                2,
                                3,
                                0
                            ],
                            "data": "29z5mr1JoRmJYQ6yp7DsrEbrPynEpLdqB3xAAZFKpw5ZW9xsJKRbWmvBmMnywCGwhSTASU8BsRoFhJTvUXdKCvgrxDh5wM",
                            "programIdIndex": 4
                        }
                    ],
                    "recentBlockhash": "2yUZchZURcMYEGSXHkXD1GnuYGs8KRW66CrDhkBbjLce"
                },
                "signatures": [
                    "279EBedXz4fLvh8iyqiP1CFqVuUh54xi1BaPjYRG6hNomjQM1xB7pEXYZEYy3TRbbfnaWoRXaqgJW4VMrPpgH1Wb"
                ]
            }
        }
```


*The entire encoded size of a Solana transaction cannot exceed 1232 bytes. 

Instruction:
- accounts indexes. variable number of elements( under 256 len? ). elements are unsigned integers 
- data of variable length. bytes for length followed by the data itself. given that max size is 1232 < 2^16 could just make the length two bytes.
- progindex. 


Inside a transaction: