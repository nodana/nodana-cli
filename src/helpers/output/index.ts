const chalk = require("chalk");

export const error = (message: string) => {
  console.log(chalk.red(message));
};

export const success = (message: string) => {
  console.log(chalk.green(message));
};

export const info = (message: string) => {
  console.log(chalk.yellow(message));
};
