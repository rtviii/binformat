
Some caveats upfront: this is a work in progress and will be versioned and evolved. The guiding principle for now (at least until Breakpoint) should be the frontend design.


## Purpose

*What purposes does this format serve? What are the intended consequences?*

- **(P)** To reduce immediate memory footprint of primary data.
- **(P)** To create useful coupling between datums(i.e. account<--> recent txs)
- **(P)** To introduce in-house landmarks/indexes(account indexes, tx numbering) into the data for ease of access. 


## Domain

Which parts of the stack does this extend to/inhabit? (Including)

- In-memory lookup table. (indexes)
- (Multiple) Separate kafka topics.
- (Multiple) Separate databases.
- Processing rust code: multiple roles (possibly, intermediary object, result,)
- Intermediate communication between back and frontend.

## Functionality

What are the operations that are going to be executed against or involving it?

*I wrote some stuff down like (write,read, update, insert, search etc.) but this needs to be articulated for each particular item in the format (account/tx/block/etc).*

```
...
```

##  Primary data structures?

This should be informed by both the domain and the functionality ideally.

```
...
```

So far we know for certain that we want to:

- search it 
- the access patterns are as random as can be
- in-memory
- should be thread-safe


# DELETE transient database notes
- To blob or not to blob: https://arxiv.org/pdf/cs/0701168.pdf
- https://arxiv.org/pdf/cs/0701168.pdf
- https://research.google/pubs/pub51/
- https://ieeexplore.ieee.org/document/6133253
- https://queue.acm.org/detail.cfm?id=1988603
- https://research.google/pubs/pub62/
- https://research.google/pubs/pub27898/
- https://research.google/pubs/pub39966/
- https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf
- https://en.wikipedia.org/wiki/Fallacies_of_distributed_computing
- https://ieeexplore.ieee.org/document/839382
- https://arxiv.org/pdf/cs/0403019.pdf

- https://github.com/tigerbeetledb/viewstamped-replication-made-famous
- https://github.com/tigerbeetledb/tigerbeetle/blob/main/docs/DESIGN.md
