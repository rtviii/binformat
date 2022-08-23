
Given that we want to play with 3 bytes of indexes and then an additional byte of additional database-specific padding for a total of *4 bytes*.

So an account like  `Vote111111111111111111111111111111111111111` might get mapped to `0x000031A1` for example.


Binary encoding:

The conundrum is to how to reconcile the fact that some accounts might be non-indexed() full-32 bytes). Ex:
```json
                    {"accountKeys": [
                        "JDuhw5kYL3rHHz6pY4GsZuqvfNe51Lpv4QufkSwXjKvW", // --> 32
                        "Fa4JCidv1WqnNAFTKxJQKHqbYLMH3vEQk8ZxPbJoTa94", // --> 32
                        "SysvarS1otHashes111111111111111111111111111",  // --> 4
                        "SysvarC1ock11111111111111111111111111111111",  // --> 4
                        "Vote111111111111111111111111111111111111111"   // --> 4
                    ],
                    
                    ...
                    }
```

It would be too easy to stick all of the same type to the front of the array: the order must be preserved between the accounts and instructions.

So we need either to 
- (1) rearrange each instruction's account indices to conform to the new 4/32 ordering of the tx accounts or  
- (2)to come up with mechanism to identify the arrangement of 4/32 accounts inside the tx array.


I'm strongly against the first option because it's a pain to maintain, collapses information (about original ordering of the accounts) post conversion and therefore has a potential to be disaster in production. 

For the second option, i think we can use the combination of *the length of the accounts array* plus a naive binary encoding of the positions of the addresses in the array with *1* being *SB-encoded* and *0* being *SB-unencoded* for the cost of additional `(ceiling(num accounts/8))` bytes right after the length-byte (the hope is this is rarely exceeds 4 bytes -- what program uses 32 accounts as input?).







```json
{ "transaction": {
                "message": {
                    "accountKeys": [
                        "JDuhw5kYL3rHHz6pY4GsZuqvfNe51Lpv4QufkSwXjKvW",
                        "Fa4JCidv1WqnNAFTKxJQKHqbYLMH3vEQk8ZxPbJoTa94",
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
                            "data": "29z5mr1JoRmJYQ6zJg9CHGgmenA3L6MvJTPz7rD2zwhmLMNsv78oAGGcxPCLGYhWT673uUjfqnEjHmzUbJGxfF1bKgVo9h",
                            "programIdIndex": 4
                        }
                    ],
                    "recentBlockhash": "36psjMqtMofrtxbd2WXaW4CA3trPxgCcMvZBuK1XR1nP"
                },
                "signatures": [
                    "4GhovD98dm5mi2JGvaKDRiTiKuRRFEMJd811p6fNNNQa3ECg6RcSoCxSW3BpPFK7zZQXTt2gADhzxssESLEDV7Mg"
                ]
            } }

```