const { instantiate } = require("@wapc/host");
const { encode, decode } = require("@msgpack/msgpack");
const { promises: fs } = require("fs");
const path = require("path");

// Argument as index 0 is the node executable, index 1 is the script filename
// Script arguments start at index 2
const scriptname = process.argv[1];
const wasmfile = process.argv[2];
const operation = process.argv[3];
const json = process.argv[4];

// If we don't have the basic arguments we need, print usage and exit.
if (!(wasmfile && operation && json)) {
  console.log("Usage: node  scriptname [wasm file] [waPC operation] [JSON input]");
  process.exit(1);
}

var host_functions = {
  'print' : print 
};

function print (payload) {

  const decoded = decode(payload);
  console.log(decoded);
}


async function main() {
  // Read wasm off the local disk as Uint8Array
  buffer = await fs.readFile(wasmfile);

  // Instantiate a WapcHost with the bytes read off disk
  const host = await instantiate(buffer,
    (binding, namespace, operation, data) => {
     
        host_functions[operation](data);
    }
    );

  // Parse the input JSON and encode as msgpack
  const payload = encode(JSON.parse(json));

  // Invoke the operation in the wasm guest
  const result = await host.invoke(operation, payload);

  // Decode the results using msgpack
  const decoded = decode(result);

  // log to the console
  console.log(`Result: ${decoded}`);
}

main().catch((err) => console.error(err));