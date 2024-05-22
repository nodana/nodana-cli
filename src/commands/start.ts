const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";

type Options = {
  key?: string;
};

export default async (id: string, options: Options) => {
  try {
    info(`Starting. Please wait...`);
    const result = await client.start(id, options);
    success(`Container started 🚀`);
    console.log("\n");
    console.log(chalk.yellow("ID:"), result.id);
    console.log(chalk.yellow("Status:"), result.status);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
