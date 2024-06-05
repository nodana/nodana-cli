const chalk = require("chalk");
import * as client from "../client";
import { error, info } from "../helpers/output";
import { getDurationString } from "../helpers/date";

type Props = {
  key?: string;
};

export default async (options: Props) => {
  try {
    const response = await client.status(options);

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (key: any) => {
  console.log("\n");
  console.log(chalk.yellow("ID:"), key.id);
  console.log(chalk.yellow("Sats remaining:"), key.sats);
};
