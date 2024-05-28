const chalk = require("chalk");
import * as client from "../client";
import { error, info } from "../helpers/output";
import { getDurationString } from "../helpers/date";

type Props = {
  key?: string;
};

export default async (options: Props) => {
  try {
    const containers = await client.list(options);

    if (containers.length === 0) {
      info("No containers found");
      return;
    }

    print(containers);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (containers: any) => {
  containers.forEach((container: any) => {
    console.log("\n");
    console.log(chalk.yellow("ID:"), container.id);
    console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
    console.log(
      chalk.yellow("Created"),
      `${getDurationString(container.created)} ago`
    );
    console.log(chalk.yellow("Fee"), `${container.fee.toLocaleString()} sats`);
    console.log(chalk.yellow("Status"), container.status);
    console.log(chalk.yellow("Version"), container.version);
  });
};
