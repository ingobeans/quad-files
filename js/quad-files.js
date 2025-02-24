var ctx = null;
var memory;

params_set_mem = function (wasm_memory, _wasm_exports) {
  memory = wasm_memory;
  ctx = {};
};

const NULL_STATE = 0;
const CANCEL_STATE = 1;
const DATA_STATE = 2;

file_buffer = null;
state = NULL_STATE;
file_name = "";
timestamp = 0;
size = 0;

function openPicker() {
  var input = document.createElement("input");
  // credit to https://codepen.io/udaymanvar/pen/MWaePBY
  input.type = "file";
  input.addEventListener("change", (_) => {
    let files = Array.from(input.files);
    if (files.length == 0) {
      state = CANCEL_STATE;
    }

    // credit to https://stackoverflow.com/a/32556944
    // File.bytes() isnt supported in chrome and we have to use one of these readers instead
    var reader = new FileReader();
    reader.onload = function () {
      var arrayBuffer = this.result;
      state = DATA_STATE;
      file_buffer = new Uint8Array(arrayBuffer);
    };
    file_name = files[0].name;
    timestamp = Math.floor(files[0].lastModified.valueOf() / 1000);
    size = files[0].size;
    reader.readAsArrayBuffer(files[0]);
  });
  input.addEventListener("cancel", (_) => {
    state = CANCEL_STATE;
  });
  input.click();
}

params_register_js_plugin = function (importObject) {
  importObject.env.quad_files_read_contents = function () {
    let object = js_object({
      state: state,
      bytes: file_buffer,
      name: file_name,
      size: size.toString(),
      timestamp: timestamp.toString(),
    });
    state = NULL_STATE;
    return object;
  };
  importObject.env.quad_files_open_dialog = function () {
    openPicker();
  };
  importObject.env.quad_files_download = function (pathObject, bytesObject) {
    let bytes = get_js_object(bytesObject);
    let path = get_js_object(pathObject);
    let mime = "application/octet-stream";
    var blob = new Blob([bytes], { type: mime });
    var objectUrl = URL.createObjectURL(blob);
    let a = document.createElement("a");
    a.href = objectUrl;
    a.download = path;
    a.click();
  };
};

miniquad_add_plugin({
  register_plugin: params_register_js_plugin,
  on_init: params_set_mem,
  name: "quad_files",
  version: "0.2.1",
});
