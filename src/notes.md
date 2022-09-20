### Getting size of value

    ```rust
    let sizeofhash = mem::size_of_val(&tx);
    println!("sizeofhash :{}", sizeofhash);

    ```

### Displaying 
```rust
let datab = "29z5mr1JoRmJYQ6yp7DsrEbrPynEpLdqB3xAAZFKpw5ZW9xsJKRbWmvBmMnywCGwhSTASU8BsRoFhJTvUXdKCvgrxDh5wM";
println!("this data is {} bytes", &datab.len());
println!("{:#2X?}", datab.as_bytes());
```