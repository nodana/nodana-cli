const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";
import { sleep } from "../helpers/date";

type Options = {
  key?: string;
};

export default async (id: string, options: Options) => {
  try {
    info(`Stopping. Please wait...`);
    const result = await client.stop(id, options);
    await sleep(5000); // wait 5 seconds instead of blocking thread on api
    success(`Container stopped ❌`);
    console.log("\n");
    console.log(chalk.yellow("ID:"), result.id);
    console.log(chalk.yellow("Status:"), result.status);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
