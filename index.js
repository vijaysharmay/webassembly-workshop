const wasm = import("./pkg");

(() => {
    wasm.then(instance => {
        console.log(instance.add(2,3));
    })
})();