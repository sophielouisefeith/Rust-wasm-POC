

//// stream compile in one go, get the lib 
WebAssembly.instantiateStreaming(fetch("utils.wasm"))
.then(results=>{
    const result = results.instance.exports.add_one(3);
    const text   = document.createTextNode(result);
    document.body.appendChild(text);
})

