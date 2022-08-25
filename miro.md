
Some caveats upfront: this is a work in progress and will be versioned and evolved. The guiding principle 



# Purpose

*What purposes does this format serve? What are the intended consequences? *

(P) To reduce immediate memory footprint of primary data.
(P) To create useful coupling between datums(i.e. account<--> recent txs)
(P) To introduce in-house landmarks/indexes(account indexes, tx numbering) into the data for ease of access. 


# Domain

Which parts of the stack does this extend to/inhabit? (Including)

- In-memory lookup table. (indexes)
- (Multiple) Separate kafka topics.
- (Multiple) Separate databases.
- Processing rust code: multiple roles (possibly, intermediary object, result,)
- Intermediate communication between back and frontend.

# Functionality

What are the operations that are going to be executed against it?

- Search
- Insert 
- Update
- Random Access



What are the primary data structures?