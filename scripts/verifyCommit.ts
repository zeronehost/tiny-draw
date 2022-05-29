import { readFileSync } from "fs";
import colors from "picocolors";

// get $1 from commit-msg script
const msgPath = process.argv[2];
const msg = readFileSync(msgPath, "utf-8").trim();

const releaseRE = /^v\d/;
const commitRE =
  /^(revert: )?(feat|fix|docs|dx|refactor|perf|test|workflow|build|ci|chore|types|wip|release|deps)(\(.+\))?: .{1,50}/;

if (!releaseRE.test(msg) && !commitRE.test(msg)) {
  console.log();
  console.error(
    `  ${colors.bgRed(colors.white(" ERROR "))} ${colors.red(
      "提交信息不合法."
    )}\n\n` +
      colors.red(
        "  正确的提交信息格式是自动生成更新日志的必要条件. 例如:\n\n"
      ) +
      `    ${colors.green("feat: add 'comments' option")}\n` +
      `    ${colors.green("fix: handle events on blur (close #28)")}\n\n` +
      colors.red("  See .github/commit-convention.md for more details.\n")
  );
  process.exit(1);
}
