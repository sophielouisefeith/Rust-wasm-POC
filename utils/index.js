

//working with a package and wasm-bindgen.
import init, {generate_secret_key} from "./pkg/utils.js";
// init()
//   .then(() => {
//       let key = generate_secret_key()
//       console.log(key);
//   });



// receives a seedphrase  to safe on local storage?
// generate a seedphrase  to safe  sent back to the browser. 
// receive the password data from the webbrowser.
// 

// on click this function needs to be runned. 
function  generatekey(){

  // if privatekey ( check local storage)
  // fil in password / fill in seedphrase

  // no private key call generate_secretkey 
  let key = generate_secret_key();
  console.log(key);

  // ask to stor and make password 
  // safe seeprhase.
  // return the prase
  
  return 
}














//working without a package and no wasm-bindgen 
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