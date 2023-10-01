const { instantiate } = require('@wapc/host');
const { encode, decode } = require('@msgpack/msgpack');
const { promises: fs } = require('fs');
const path = require('path');

var storage = {};

var host_functions = {
  'wapc-env-print' : print,
  'prover-storage-insert': prover_storage_insert,
  'prover-storage-get': prover_storage_get,
  'verifier-storage-get': verifier_storage_get
};

function verifier_storage_get (payload) {
  const id = decode(payload);
  console.log('Get: ',id);
  console.log(storage[id]);
  return storage[id];

}

function prover_storage_get (payload) {
  const id = decode(payload);
  console.log('Get: ',id);
  console.log(storage[id]);
  return storage[id];

}

function prover_storage_insert (payload) {

  const decoded = decode(payload);
  const [id,authepub] = decoded

  console.log('Id: ',id)
  console.log('Authepub: ',authepub)

  storage[id] = payload

}


function print (payload) {

  const decoded = decode(payload);
  console.log(decoded);
}

async function main() {
  // Read wasm off the local disk as Uint8Array
  buffer = await fs.readFile(path.join(__dirname, '../../../target/wasm32-unknown-unknown/debug', 'epubcontract.wasm'));

  epubsource = await fs.readFile( path.join(__dirname, '../assets', 'SagradaBiblia_2.epub'));

  // Instantiate a WapcHost with the bytes read off disk
  const host = await instantiate(buffer,
        (binding, namespace, operation, data) => {
            const fname = binding + '-' + namespace + '-' + operation;
            return host_functions[fname](data)
        }
        );

  // Encode the payload with MessagePack
  const payload = encode(epubsource);

  console.log("Tryint to register...");
  // Invoke the operation in the wasm guest
  const result = await host.invoke('prover::register', payload);
  const id = decode(result);

  //console.log(`Result: ${result}`);

  // Decode the results using MessagePack
  const signature = Buffer.from(id).toString('hex') ;

  // log to the console
  console.log(`EPub Signature: ${signature}`);
  
  const manifest = await host.invoke('prover::manifest',encode({data: id}));
  console.log('Raw Manifest computation: ',manifest);

  const decoded_computation = decode(manifest);
  console.log('Decoded computation: ',decoded_computation);

  console.log(`EPub Manifest: ${decoded_computation.result}`);
  console.log(`EPub Manifest number of proofs: ${decoded_computation.proofs.length}`);
  console.log(Array.isArray(decoded_computation.proofs));

  const v_encoded = await host.invoke('verifier::manifest',encode([{data: id},decoded_computation.proofs]));

  const v = decode(v_encoded);

  console.log(`Verifier result: ${v.result}`);
  console.log(`Verifier proofs: ${v.proofs}`);

  const encoder = new TextEncoder();
  const resource_name = encoder.encode('EPUB/cover.xhtml');

  const resource_encoded = await host.invoke('prover::resource',encode([{data: id},Array.from(resource_name)]));
  const resource = decode(resource_encoded);

  console.log(`Resource result: ${String.fromCharCode.apply(null,resource.result[1][1][0])}`);
  console.log('Resource proofs');
  console.log(resource.proofs);

  /*
  const search_resource = encoder.encode('EPUB/s04.xhtml');
  const search_selector = encoder.encode('#pgepubid00498');
  const search_encoded = await host.invoke('prover::search',encode([{data: id},Array.from(search_resource),Array.from(search_selector)]));
  const search = decode(search_encoded);

  console.log('Search:');
  console.log(search.result[1][1]);
  */

  const metadata = await host.invoke('prover::metadata',encode({data: id}));

  const decoded_metadata = decode(metadata);
  console.log('Decoded computation Metadata: ',decoded_metadata);

  const cover = await host.invoke('prover::cover',encode({data: id}));

  const decoded_cover = decode(cover);
  console.log('Decoded computation Cover: ',decoded_cover);  

  const locate_resource = encoder.encode('EPUB/s04.xhtml');
  const locate_media = encoder.encode('text/html');
  const from_selector = encoder.encode('html body #pgepubid00492 #pgepubid00498 div');
  const to_selector = encoder.encode('html body #pgepubid00492 #pgepubid00501 h3');


  const locate_encoded = await host.invoke('prover::locate',encode([{data: id},[locate_resource,locate_media,from_selector,to_selector]]));
  const locate = decode(locate_encoded);

  console.log('Locate:');
  console.log(locate.result[1][1]);

}

main().catch(err => console.error(err));
