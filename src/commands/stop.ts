const chalk = require("chalk");
import * as client from "../client";
import { error, info } from "../helpers/output";
import { sleep } from "../helpers/date";

type Options = {
  key?: string;
};

const DELAY = 5000;

export default async (id: string, options: Options) => {
  try {
    info(`Stopping. Please wait...`);
    const response = await client.stop(id, options);
    await sleep(DELAY); // wait 5 seconds instead of blocking thread on api

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  info(`Container stopped ❌`);
  console.log("\n");
  console.log(chalk.yellow("ID:"), data.id);
  console.log(chalk.yellow("Status:"), data.status);
  console.log("\n");
};
