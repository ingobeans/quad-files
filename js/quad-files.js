var ctx = null;
var memory;

params_set_mem = function (wasm_memory, _wasm_exports) {
  memory = wasm_memory;
  ctx = {};
};

const NO_DATA_SIGNATURE = [1, 48, 90];

file_buffer = NO_DATA_SIGNATURE;

function openPicker() {
  var input = document.createElement("input");
  // credit to https://codepen.io/udaymanvar/pen/MWaePBY
  input.type = "file";
  input.onchange = (_) => {
    let files = Array.from(input.files);
    console.log(files);
    files[0].bytes().then((bytes) => {
      console.log(bytes);
      file_buffer = bytes;
    });
  };
  input.click();
}

params_register_js_plugin = function (importObject) {
  importObject.env.quad_files_read_contents = function () {
    console.log("quad_files_open_dialog called");
    let data = file_buffer;
    file_buffer = NO_DATA_SIGNATURE;
    return js_object(data);
  };
  importObject.env.quad_files_open_dialog = function () {
    openPicker();
  };
};

miniquad_add_plugin({
  register_plugin: params_register_js_plugin,
  on_init: params_set_mem,
  name: "quad_files",
  version: "8",
});
