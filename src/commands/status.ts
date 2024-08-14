const chalk = require("chalk");
import * as client from "../client";
import { error } from "../helpers/output";
import { getDurationString } from "../helpers/date";

type Props = {
  key?: string;
};

export default async () => {
  try {
    const response = await client.status();

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (key: any) => {
  console.log("\n");
  console.log(chalk.yellow("ID:"), key.id);
  console.log(chalk.yellow("Sats Remaining:"), key.sats);
  console.log(chalk.yellow("Age"), getDurationString(key.age));
};
