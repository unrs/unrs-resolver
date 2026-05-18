import fs from "node:fs";

const filename = "./napi/index.js";
let data = fs.readFileSync(filename, "utf-8");
data = data.replace(
  "\nif (!nativeBinding) {",
  (s) =>
    `
if (!nativeBinding && globalThis.process?.versions?.["webcontainer"]) {
  try {
    nativeBinding = require('napi-postinstall/fallback')(require.resolve('./package.json'), true);
  } catch (err) {
    loadErrors.push(err)
  }
}
` + s,
);
data =
  data +
  `
if (process.versions.pnp) {
  process.env.UNRS_RESOLVER_YARN_PNP = '1'
}
`;
fs.writeFileSync(filename, data);
