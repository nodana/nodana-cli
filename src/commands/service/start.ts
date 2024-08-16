const chalk = require("chalk");
import * as client from "../../client";
import { error, info, success } from "../../helpers/output";

export default async (id: string) => {
  try {
    info(`Starting. Please wait...`);
    const response = await client.startService(id);

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  success(`Service started 🚀`);
  console.log("\n");
  console.log(chalk.yellow("Id:"), data.id);
  console.log("\n");
};
