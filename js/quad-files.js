var ctx = null;
var memory;

params_set_mem = function (wasm_memory, _wasm_exports) {
  memory = wasm_memory;
  ctx = {};
};

const NULL_SIGNATURE = [1, 48, 90];
const CANCEL_SIGNATURE = [1, 48, 91];

file_buffer = NULL_SIGNATURE;

function openPicker() {
  var input = document.createElement("input");
  // credit to https://codepen.io/udaymanvar/pen/MWaePBY
  input.type = "file";
  input.addEventListener("change", (_) => {
    let files = Array.from(input.files);
    if (files.length == 0) {
      file_buffer = CANCEL_SIGNATURE;
    }
    files[0].bytes().then((bytes) => {
      file_buffer = bytes;
    });
  });
  input.addEventListener("cancel", (_) => {
    file_buffer = CANCEL_SIGNATURE;
  });
  input.click();
}

params_register_js_plugin = function (importObject) {
  importObject.env.quad_files_read_contents = function () {
    let data = file_buffer;
    file_buffer = NULL_SIGNATURE;
    return js_object(data);
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
  version: "0.1.3",
});
