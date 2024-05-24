const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";

type Options = {
  key?: string;
};

export default async (id: string, options: Options) => {
  try {
    info(`Starting. Please wait...`);
    const response = await client.start(id, options);

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  success(`Container started 🚀`);
  console.log("\n");
  console.log(chalk.yellow("ID:"), data.id);
  console.log(chalk.yellow("Status:"), data.status);
  console.log("\n");
};
