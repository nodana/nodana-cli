const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";
import { read as readFile } from "../helpers/file";

type Props = {
  key?: string;
};

export default async (options: Props) => {
  try {
    const containers = await client.list(options);

    if (containers.length === 0) {
      console.log("\n");
      info("No containers created");
      console.log("\n");
      return;
    }

    const [container] = containers;

    console.log("\n");
    console.log(chalk.yellow("ID:"), container.id);
    console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
    console.log(chalk.yellow("Created"), `${container.created} minutes ago`);
    console.log(chalk.yellow("Fee"), `${container.fee} sats`);
    console.log(chalk.yellow("Status"), container.status);
    console.log(chalk.yellow("Version"), container.version);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
