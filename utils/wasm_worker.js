


// write to our buffer


//// stream compile in one go, get the lib 
WebAssembly.instantiateStreaming(fetch("utils.gc.wasm"))
.then(wasmModule=>{
    const result = wasmModule.instance.exports.add_one(3);
    const text   = document.createTextNode(result);
    document.body.appendChild(text);
})

