const chalk = require("chalk");
import * as client from "../client";
import { error, info } from "../helpers/output";
import { read as readFile } from "../helpers/file";

type Options = {
  key?: string;
};

export default async (id: string, { key }: Options) => {
  try {
    let apiKey;

    try {
      apiKey = key || (await readFile());
    } catch (e: any) {
      throw new Error("Nodana config file not found");
    }

    info(`Starting. Please wait...`);
    const result = await client.start(apiKey, id);
    console.log("\n");
    console.log(chalk.yellow("ID:"), result.id);
    console.log(chalk.yellow("Status:"), result.status);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
