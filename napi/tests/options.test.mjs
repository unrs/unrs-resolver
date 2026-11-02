import * as path from "node:path";
import { assert, describe, it } from "vite-plus/test";

import { ResolverFactory } from "../index.js";
import { normalizePath } from "./utils.mjs";

const fixtureDir = path.resolve(
  import.meta.dirname,
  "../../fixtures/enhanced-resolve/test/fixtures",
);

describe("option", () => {
  describe("aliasFields", () => {
    it("should allow field string ", () => {
      const resolver = new ResolverFactory({ aliasFields: ["browser"] });
      assert.match(
        normalizePath(resolver.sync(fixtureDir, "./browser-module/lib/replaced.js").path),
        /browser-module\/lib\/browser\.js$/,
      );
    });
    it("should allow json path array", () => {
      const resolver = new ResolverFactory({
        aliasFields: [["innerBrowser1", "field", "browser"]],
      });

      assert.match(
        normalizePath(resolver.sync(fixtureDir, "./browser-module/lib/main1.js").path),
        /browser-module\/lib\/main\.js$/,
      );
    });
  });

  describe("exportsFields", () => {
    const createTest = (exportsFields) => {
      const resolver = new ResolverFactory({ exportsFields });
      assert.match(
        normalizePath(
          resolver.sync(path.resolve(fixtureDir, "./exports-field3"), "exports-field").path,
        ),
        /\/exports-field\/src\/index\.js$/,
      );
    };
    it("should allow string as field item", () => createTest(["broken"]));
    it("should allow json path array as field item", () => createTest([["broken"]]));
  });

  describe("mainFields", () => {
    const createTest = (mainFields) => {
      const resolver = new ResolverFactory({ mainFields });
      assert.match(normalizePath(resolver.sync(fixtureDir, "../..").path), /\/lib\/index\.js$/);
    };
    it("should use `'main'` as default", () => createTest(undefined));
    it("should allow field string", () => createTest("main"));
    it("should allow field array", () => createTest(["main"]));
  });
});

describe("nodeCompat", () => {
  const dir = path.join(fixtureDir, "prefer-file-over-dir");
  const file = path.join(dir, "node_modules", "bar.js");
  const index = path.join(dir, "node_modules", "bar", "index.js");
  const resolveBar = (nodeCompat) =>
    normalizePath(new ResolverFactory({ nodeCompat }).sync(dir, "bar").path);

  it("should enable both switches by default", () => {
    assert.equal(resolveBar(undefined), normalizePath(file));
    assert.equal(resolveBar(true), normalizePath(file));
  });

  it("should disable both switches with false", () => {
    assert.equal(resolveBar(false), normalizePath(index));
  });

  it("should treat the object form as a whitelist", () => {
    assert.equal(resolveBar({}), normalizePath(index));
    assert.equal(resolveBar({ entrySymlinks: true }), normalizePath(index));
    assert.equal(resolveBar({ fileFirst: true }), normalizePath(file));
    assert.equal(
      resolveBar({ entrySymlinks: true, fileFirst: false }),
      normalizePath(index),
    );
  });
});
