

working with a package.
import init, {generate_secret_key} from "./pkg/utils.js";
init()
  .then(() => {
      let key = generate_secret_key()
      console.log(key);
  });



  // //// stream compile in one go, get the lib 
// WebAssembly.instantiateStreaming(fetch("utils.wasm", {}))
// .then(obj=>{

//     // var jsarray = new Uint8array(results.instance.exports.memory.buffer);
//     // jsarray = obj.instance.exports.generate_secret_key();

   
    
//     // console.log(jsarray);
//     const results = obj.instance.exports.add_one(3);
//     const text   = document.createTextNode(result);
//     document.body.appendChild(text);
// })


// import init from "./pkg/utils.js";

// const runWasm = async () => {

//     const bytes = await init("./pkg/hello_world_bg.wasm");
//     const result = bytes.generate_secret_key();

//     document.body.textContent = `add secretkey ${result}`;
//     console.log(result);
// };


// function  generatekey(){

//   // if privatekey ( check local storage)

//   // no private key call generate_secretkey 
//   // return the prase
//   // ask for password. 
//   return 
// }

