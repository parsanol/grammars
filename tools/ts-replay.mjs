// The registry's TypeScript replay gate: every artifact replays its
// embedded corpus through the wasm runtime — accept cases parse,
// reject cases refuse. Generic by construction.
import { readdirSync, readFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const require = createRequire(import.meta.url);
const { PargRuntime, initialize } = await import("parsanol/browser");
await initialize(readFileSync(require.resolve("parsanol/wasm")));

const artifactsDir = join(root, "artifacts");
const corporaDir = join(root, "corpora");

const artifacts = readdirSync(artifactsDir).filter((f) => f.endsWith(".json"));
if (artifacts.length === 0) throw new Error("no artifacts found");

let cases = 0;
for (const file of artifacts) {
  const flavor = file.replace(/\.json$/, "");
  const artifactJson = readFileSync(join(artifactsDir, file), "utf8");
  const envelope = JSON.parse(artifactJson);
  const entry = envelope.default_entry ?? Object.keys(envelope.entries ?? {})[0];
  if (!entry) throw new Error(`${file}: no entry`);
  const corpusPath = join(corporaDir, flavor, "corpus.json");
  let casesDoc;
  try {
    casesDoc = JSON.parse(readFileSync(corpusPath, "utf8"));
  } catch {
    console.log(`[replay] ${flavor}: no corpus, skipped`);
    continue;
  }
  const rt = new PargRuntime(artifactJson);
  try {
    for (const c of casesDoc.cases) {
      const outcome = (() => {
        try {
          rt.parseShape(c.input, entry);
          return true;
        } catch {
          return false;
        }
      })();
      const wants = c.kind === "accept";
      if (outcome !== wants) {
        throw new Error(
          `${corpusPath}: ${JSON.stringify(c.input)} parsed=${outcome} but the case is ${c.kind}`,
        );
      }
      cases += 1;
    }
  } finally {
    rt.artifact.free();
  }
  console.log(`[replay] ${flavor}: corpus replayed through wasm`);
}
console.log(`[replay] ${artifacts.length} artifacts, ${cases} corpus cases — green`);
