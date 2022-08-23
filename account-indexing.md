
Given that we want to play with 3 bytes of indexes and then a byte of additional database-specific padding for a total of *4 bytes*, an account like  `Vote111111111111111111111111111111111111111` might get mapped to `0x000031A1` for example.

The problem is to how to reconcile the fact that some accounts might be non-indexed() full-32 bytes). Ex:
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

For the second option, i think we can use the combination of *the length of the accounts array* plus a naive binary encoding of the positions of the addresses in the array with *1* being *SB-indexed* and *0* being *SB-indexed* for the cost of additional `(ceiling(num accounts/8))` bytes right after the length-byte (the hope is this is rarely exceeds 4 bytes -- what program uses 32 accounts as input?).


Ex. (contrived) the following translates to `[0x05][0x0c]`. There are `5` accounts, the indexed pattern is `01011`, which, left-zero-padded to a byte, is `0b00001011` == `0x0c`.
```bash
                        "JDuhw5kYL3rHHz6pY4GsZuqvfNe51Lpv4QufkSwXjKvW", // unindexed
                        "Fa4JCidv1WqnNAFTKxJQKHqbYLMH3vEQk8ZxPbJoTa94", // indexed
                        "SysvarS1otHashes111111111111111111111111111",  // unindexed
                        "SysvarC1ock11111111111111111111111111111111",  // indexed
                        "Vote111111111111111111111111111111111111111"   // indexed
```
This introduces the overhead of needing to look up the ordering first to index into the accounts array correctly when pulling up the address itself, but we really want this 32->4 saving across the board.






Then, the (v1 unimplemented/OLD) encoding:
```rust
FLAG_TX_START       : = [ 9 bytes: 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11 ]
ACCOUNT_ADDRESSES   : = [ `acc_len:` 1 byte ][V(`acc_len)` * 32 bytes ]
HEADER              : = [ 3 bytes]
TX_NUMBER           : = [ 8 bytes]
SIGNATURES          : = [ `sigs_num:1` byte] [V(`signs_num)` * 64 bytes ]
INSTRUCTIONS        : = [ `ixs_len:` 2 bytes][V(`ixs_len)` ]
```

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