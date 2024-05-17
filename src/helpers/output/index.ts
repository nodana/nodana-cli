const chalk = require("chalk");

export const error = (message: string) => {
  console.log(chalk.red("ERROR: ", message));
};

export const success = (message: string) => {
  console.log(chalk.green("SUCCESS: ", message));
};
