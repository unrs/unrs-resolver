import { patch } from "napi-postinstall/patch";

// Inserts the all-platform runtime fallback (no WebContainer-only guard) before the
// loader's final `if (!nativeBinding)` check, guarded by `SKIP_UNRS_RESOLVER_FALLBACK`.
// It replaces the old postinstall script, so installs that skip build scripts (pnpm 10+,
// `--ignore-scripts`, npm RFC #868) still get a working binding on first require.
patch({
  entry: "./napi/index.js",
  name: "unrs-resolver",
  append: `if (process.versions.pnp) {
  process.env.UNRS_RESOLVER_YARN_PNP = '1'
}
`,
});
