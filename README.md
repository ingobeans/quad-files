# quad files

![image](https://github.com/user-attachments/assets/419f40dc-68e8-4372-87a0-9d47461fd0a2)


plugin to make a file picker. install by adding the js file to your html after gl.js is loaded. in your rust code you can install this crate and use `open_dialog()` and `read_contents()` to open a file dialog and read contents.

they're split in two functions because the js side would require async, which didn't behave well with the rust code. instead you first have to call `open_dialog()`, and keep calling `read_contents()` until something is read.

check out the example code.
