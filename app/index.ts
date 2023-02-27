import "./index.scss";

// https://developer.mozilla.org/zh-CN/docs/WebAssembly/Loading_and_running
import("../wasm/pkg").then((module) => {
  console.log("app");
  const add = module.add;
  console.log(add(1, 5));
});
