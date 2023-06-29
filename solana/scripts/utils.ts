import chalk from "chalk";

export function errorHandler<T extends Error>(error: T) {
  if (error["code"] && error["msg"]) {
    console.error("\n" + chalk.red(`Error ${error["code"]}: ${error["msg"]}`));

    if (error["logs"]) {
      console.error("\n" + chalk.gray(error["logs"].join("\n")));
    }
  } else {
    console.error("\n" + chalk.red(error));
  }

  process.exit(1);
}

export function successHandler(output: String | object) {
  if (typeof output === "string") {
    console.log(chalk.green(`\nSuccess: ${output}`));
  } else if (typeof output === "object") {
    console.log(JSON.stringify(output, undefined, 2));
  }

  process.exit(0);
}
