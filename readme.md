
# Binary Block Format

Lets define _V(x)_ to be a _variable-length_ array of length up-to and including _x_. Then _V(1232)_ is byte array that can be anywhere _from 0 to 1232 bytes long_(inclusive).

Building the format bottom up.


## Instruction

Sample instruction looks like this:
```json

                            "programIdIndex": 4
                            "accounts"      : [1,2,3,0],
                            "data"          :"29z5mr1JoRmJYQ6yp7DsrEbrPynEpLdqB3xAAZFKpw5ZW9xsJKRbWmvBmMnywCGwhSTASU8BsRoFhJTvUXdKCvgrxDh5wM",

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
+ Data array begins at ( 4 + acc_ix_len + 1 )st byte.


So, in the end:

```rust
PROGRAM_INDEX       := [ 0x04 ]                                                # <----- prog_ix
ACCOUNT_INDEXES_LEN := [ 0x04 ]                                                # <----- accixs.len
DATA_LEN            := [ 0x00, 0x5e]                                           # <----- ixdata.len
ACCOUNT_INDEXES     := [ 0x01, 0x02,0x03, 0x00 ]                               # <----- accixs
DATA                := [ 0x32, 0x39, 0x7A, 0x35, 0x6D, 0x72, 0x31, 0x4A, 0x6F, # .
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





## Transaction :

```bash
accounts            ---------------------------- [num accounts: 1 byte][num x 32bytes]
header              ---------------------------- [byte][byte][byte]
signatures          ---------------------------- [n signatures x 32 bytes]
blockhash           ---------------------------- [32bytes]
n instructions of different lengths:
```


Then, the encoding:
```rust
FLAG_TX_START       : = [ 4 bytes: 0x00, 0x00, 0x00, 0x00]
ACCOUNT_ADDRESSES   : = [ acc_len: 1 byte ][V(acc_len * 32 bytes )]
HEADER              : = [ 3 bytes]
SIGNATURES          : = [ sigs_num:1 byte] [V(signs_num * 64 bytes )]
INSTRUCTIONS        : = [ ixs_len: 2 bytes][V(ixs_len) ]
```

The padding is there to signify the beggining of a transaction. This way, when we look for an account match in the transaction and end up in the middle of the block, we can always reorient ourselves by tracking back to the nearest `FLAG_TX_START`. Furthermore, if we replace (some) of the addresses with custom indexes, this flag would be the the anchor to which the accounts latch and can be extended to the hybrid custom indexes + vanilla addresses solution. See the [trick](#primes-trick) below.





### General Notes

*The entire encoded size of a Solana transaction cannot exceed 1232 bytes.

- It'd be sure nice to know the average number of ix/tx and tx/block.

- Both the instructions and transaction arrays can be sorted by length with the smallest coming in the front to minimize jump lenths in the case of seeks.

- How many 0-bytes is really enough to eliminate collision? How wide should the flag really be?

- is repeated indexing and occasional summation more costly or inserting flags everywhere?

- we could put all the accounts mentioned in a block to the top of the block, but let's not overcomplicate this for now, especially given that we might want to later stream transactions by themselves, without blocks.



- Seek/read considerations:


    Next, your disk can probably read sequential data at around 100 megabytes/second; that is, it can read 1 megabyte sequentially in around the same time it takes to perform a seek. So if two of your values are less than 1 megabyte apart, you are better off reading all of the data between them than performing the seek between them. (But benchmark this to find the optimal trade-off on your hardware.)


### Primes trick 

- not sure when yet, but for certain cases where order needs to be preserved perhaps we can use a prime-number factorization method confined to 8bytes. I.e. in the case of hybrid custom-index-vanilla-address approach we can signify at which positions in the address array the addresses reside (given that they will be more numerous(?)) by assigning a prime number to each position in the address array, multiplying the positions of vanilla addresses and storing the product. everything else will be considered a custom index and will be interpreted as 4-byte number or whatever (instaed of 32).



https://stackoverflow.com/questions/323604/what-are-important-points-when-designing-a-binary-file-format
https://stackoverflow.com/questions/6651503/random-access-of-a-large-binary-file

### Other 

Sample transaction:

Sample transaction:

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
                                                        "programIdIndex": 3,
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

```