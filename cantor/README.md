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

f = ["r", "e", "v", "e", "r", "s", "e", " ", "t", "h", "i", "s"]
f.join("")
  .split(" ")
  .reverse()
  .map(w => w.split(""))
  .reduce((acc, n) => acc.concat(n))

r = [list(word) for word in "".join(f).split(" ")[::-1]]
map(lambda word: list(word), "".join(f).split(" ")[::-1])
reduce(lambda acc, n: acc + n, [list(word) for word in "".join(f).split(" ")[::-1]])
