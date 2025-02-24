# quad files

![image](https://github.com/user-attachments/assets/419f40dc-68e8-4372-87a0-9d47461fd0a2)

macro/miniquad plugin for file dialogs. cross platform, uses RFD for non wasm targets. on wasm, uses a dumb little plugin i wrote.

## features

- opening files
- downloading files

## usage

### saving/downloading file

use `download(filename: &str, bytes: &[u8], filter: Option<&str>)` to download a file.

`filename` is requested file name

`bytes` is file data

if `filter` is Some, only show files of the same type in the file picker. The &str contained will be the name of the filter

### opening file picker

you can create a `FilePicker` object for opening files. you have to first call the method `open_dialog()` to open the file picker, then call `read_contents()` until data is available to read.
they're split in two functions because the js side would require async, which didn't behave well with the rust code.
example:

```rust
let mut dialog = FilePicker::new();
dialog.open_dialog();
loop {
    if let FileInputResult::Data(_data) = dialog.update() {
        println!("got file data!");
    }
}
```

## install

for WASM, remember to add the js file(s) to your html after gl.js is loaded.

```html
<script src="https://cdn.jsdelivr.net/gh/not-fl3/sapp-jsutils/js/sapp_jsutils.js"></script>
<script src="https://cdn.jsdelivr.net/gh/ingobeans/quad-files@53467a770a68c9f225f3e7465a3e6b2d076a8752/js/quad-files.js"></script>
```

## docs?

check out the example code, silly. you can build it with: `cargo build --release --target wasm32-unknown-unknown --example demo`.

you can serve with something like `basic-http-server` in root and opening `http://127.0.0.1:4000/examples/`
