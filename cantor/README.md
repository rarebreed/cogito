# cantor

cantor defines an attribute macro that can be use to inject the fields of one struct into another
struct.  It will also auto generate traits in the subset struct for the superset.  For example,
imagine you have the following code that needs to be serialized:

```rust
struct Message<T> {
    from: String,
    to: String,
    data: T
}
```
