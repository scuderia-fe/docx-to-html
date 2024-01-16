import fs from "fs";
import { join } from "path";

const packageJson = fs.readFileSync(
  join(process.cwd(), "pkg", "package.json"),
  "utf8"
);

const pkgJson = JSON.parse(packageJson);
pkgJson.name = "@scuderia-fe/docx-to-html";

fs.writeFileSync(
  join(process.cwd(), "pkg", "package.json"),
  JSON.stringify(pkgJson, null, 2),
  "utf8"
);
