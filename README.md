# `some-mut`

A utility library that mainly lets you access a `Some` and then `take()` it infallibly.

Useful, for example, in a `Future` implementation, when you might re-enter into
a function multiple times and so can't `take()` until a sub-future is `Ready`:

```rust
// for a theoretical `StreamExt::forward()`/`SinkExt::send_all()` implementation:
fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
    if let Some(buffered_item) = self.buffered_item.some_mut() {
        ready!(self.sink.poll_ready(cx))?;
        self.sink.start_send(buffered_item.take())?;
    }
    // ...
}
```

## License

This project is licensed under the MIT license. Please see the
[LICENSE](LICENSE) file for more details.
