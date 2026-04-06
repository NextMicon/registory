import { exec } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync } from "node:fs";
import { mkdir, readFile, readdir, stat, writeFile } from "node:fs/promises";
import { basename, join } from "node:path";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";
import { stringify as stringifyYaml } from "yaml";

const execAsync = promisify(exec);

// ---------------------------------------------------------------------------

async function main() {
  const root = join(fileURLToPath(import.meta.url), "..");
  const dist = join(root, "dist");

  // --- 1. Package ---

  await mkdir(join(dist, "package"), { recursive: true });
  const packages = await processEntries(
    join(root, "package"),
    join(dist, "package"),
    "project.nm.yaml",
  );

  // --- 2. Platform ---

  await mkdir(join(dist, "platform"), { recursive: true });
  const platforms = await processEntries(
    join(root, "platform"),
    join(dist, "platform"),
    "platform.nm.yaml",
  );

  // --- 3. Index ---
  const index = { packages, platforms };

  await Promise.all([
    writeFile(join(dist, "index.yaml"), stringifyYaml(index)),
    writeFile(
      join(dist, "README.md"),
      [
        "# NextMicon Package Registry",
        "",
        "## Packages",
        "",
        "| Archive | Hash |",
        "|---------|------|",
        ...index.packages.map(({ name, hash }) => `| [${name}](./package/${name}.tar.gz) | \`${hash}\` |`),
        "",
        "## Platforms",
        "",
        "| Archive | Hash |",
        "|---------|------|",
        ...index.platforms.map(({ name, hash }) => `| [${name}](./platform/${name}.tar.gz) | \`${hash}\` |`),
        "",
      ].join("\n"),
    ),
  ]);
}

async function processEntries(src: string, dst: string, meta: string): Promise<{ name: string; hash: string }[]> {
  const tasks: Promise<{ name: string; hash: string } | null>[] = [];

  for (const pkg of await sortedDirs(src)) {
    for (const ver of await sortedDirs(pkg)) {
      if (!existsSync(join(ver, meta))) {
        console.log(`\x1b[31m✗ ${basename(pkg)}:${basename(ver)} (no ${meta})\x1b[0m`);
        continue;
      }
      const name = `${basename(pkg)}:${basename(ver)}`;
      const archivePath = join(dst, `${name}.tar.gz`);
      tasks.push(
        archive(ver, archivePath).then(async () => {
          const hash = await sha256(archivePath);
          console.log(`\x1b[32m✓ ${name} : ${hash}\x1b[0m`);
          return { name, hash };
        }),
      );
    }
  }

  const results = await Promise.all(tasks);
  return results.filter((r) => r !== null);
}

// ---------------------------------------------------------------------------

const sha256 = async (path: string): Promise<string> =>
  createHash("sha256").update(await readFile(path)).digest("hex");

const sortedDirs = async (dir: string): Promise<string[]> => {
  if (!existsSync(dir)) return [];
  const entries = await readdir(dir);
  const paths = entries.sort().map((d) => join(dir, d));
  const stats = await Promise.all(paths.map((p) => stat(p)));
  return paths.filter((_, i) => stats[i].isDirectory());
};

const archive = async (src: string, dst: string): Promise<void> => {
  await execAsync(`tar -czf "${dst}" -C "${src}" .`);
};

// ---------------------------------------------------------------------------

main();
