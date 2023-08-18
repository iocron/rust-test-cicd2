# ToDos
- Publish rust/wasm as own npm package

## Docs
- https://nodejs.dev/en/learn/nodejs-with-webassembly/
- https://rustwasm.github.io/docs/wasm-pack/quickstart.html

## Notice
For a existing rust package/project use the following steps to publish a rust-node npm package (otherwise create a new project by using `wasm-pack new my-new-rust-wasm-project`): 

1. cargo install wasm-pack
2. Add the following to your Cargo.toml: 
    ```
    [dependencies]
    wasm-bindgen = "0.2"

    [lib]
    crate-type = ["cdylib", "rlib"]
    ```
3. Try your first build: `wasm-pack build`
4. Publish your build if ready: `wasm-pack publish`
