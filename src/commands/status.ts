const chalk = require("chalk");
import * as client from "../client";
import { error } from "../helpers/output";
import { getDurationString } from "../helpers/date";

type Props = {
  key?: string;
};

export default async (options: Props) => {
  try {
    const response = await client.status(options);

    print(response);
  } catch (e: any) {
    error("Status command could not be completed");
  }
};

const print = (key: any) => {
  console.log("\n");
  console.log(chalk.yellow("ID:"), key.id);
  console.log(chalk.yellow("Sats Remaining:"), key.sats);
  console.log(chalk.yellow("Age"), getDurationString(key.age));
  console.log(chalk.yellow("Beta"), key.beta);
};
