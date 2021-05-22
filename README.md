# Web Sys Demo

A demo project using wasm-bindgen and web-sys to draw on a canvas.


Learn more:

- [https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html)
- [https://rhmoller.dev/posts/2020/my-experience-with-rust-and-websys/](https://rhmoller.dev/posts/2020/my-experience-with-rust-and-websys/)

### Dependencies

```
$ cargo install wasm-pack
$ cargo install cargo-make
$ cargo install cargo-script
```

### Build and run

```
$ cargo make all
$ python3 -m http.server 8080
```
