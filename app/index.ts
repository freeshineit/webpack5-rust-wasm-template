import "./index.scss";

// https://developer.mozilla.org/zh-CN/docs/WebAssembly/Loading_and_running
import("../wasm/pkg").then((module) => {
  console.log("app");

  // +
  const add = module.addition;
  console.log("1 + 5 = ", add(1, 5));

  // -
  const sub = module.subtraction;
  console.log("10 - 9 = ", sub(10, 9));

  // *
  const multi = module.multiplication;
  console.log("5 * 2 = ", multi(5, 2));

  // /
  const div = module.division;
  console.log("10 / 2 = ", div(10, 2));
});
