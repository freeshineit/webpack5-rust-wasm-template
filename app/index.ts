import "./index.scss";

// https://developer.mozilla.org/zh-CN/docs/WebAssembly/Loading_and_running
import("../wasm/pkg").then((module) => {
  console.log("app");
  const add = module.addition;
  console.log("1 + 5 = ", add(1, 5));

  const sub = console.log("10 - 1 = ", module.subtraction(10, 9));
});
