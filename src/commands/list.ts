const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";
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

    const [container] = containers;
    print(container);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  console.log("\n");
  console.log(chalk.yellow("ID:"), data.id);
  console.log(chalk.yellow("Connection Url:"), data.connectionUrl);
  console.log(
    chalk.yellow("Created"),
    `${getDurationString(data.created)} ago`
  );
  console.log(chalk.yellow("Fee"), `${data.fee.toLocaleString()} sats`);
  console.log(chalk.yellow("Status"), data.status);
  console.log(chalk.yellow("Version"), data.version);
  console.log("\n");
};
