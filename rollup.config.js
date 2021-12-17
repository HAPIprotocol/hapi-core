import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import dts from "rollup-plugin-dts";

export default [
  {
    input: ["out-tsc/lib/index.js"],
    external: ["@solana/web3.js", "bn.js", "borsh", "bs58"],
    output: [
      {
        file: "out-lib/index.esm.js",
        format: "esm",
        sourcemap: true,
      },
    ],
    plugins: [resolve(), commonjs(), json()],
  },
  {
    input: ["out-tsc/lib/index.js"],
    external: ["@solana/web3.js", "bn.js", "borsh", "bs58"],
    output: [
      {
        file: "out-lib/index.cjs.js",
        format: "commonjs",
        sourcemap: true,
      },
    ],
    plugins: [resolve(), commonjs(), json()],
  },
  {
    input: "out-tsc/lib/index.d.ts",
    output: [{ file: "out-lib/index.d.ts", format: "es" }],
    plugins: [dts()],
  },
];
