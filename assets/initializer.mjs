export default function myInitializer () {
  const progress = document.getElementById("progress");
  return {
    onStart: () => {
      console.log("Loading...");
      console.time("trunk-initializer");
    },
    onProgress: ({current, total}) => {
      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        let value = Math.round((current/total) * 100);
        console.log("Loading...", value, "%" )
        progress.value = value
      }
    },
    onComplete: () => {
      console.log("Loading... done!");
      console.timeEnd("trunk-initializer");
    },
    onSuccess: (wasm) => {
      console.log("Loading... successful!");
      console.log("WebAssembly: ", wasm);
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    }
  }
};
