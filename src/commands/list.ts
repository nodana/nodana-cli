const chalk = require("chalk");
import * as client from "../client";
import { error, info, success } from "../helpers/output";
import { read as readFile } from "../helpers/file";

type Props = {
  key: string;
};

export default async ({ key }: Props) => {
  try {
    const apiKey = key || (await readFile());

    const containers = await client.list(apiKey);

    if (containers.length === 0) {
      console.log("\n");
      info("No containers running");
      console.log("\n");
      return;
    }

    const [container] = containers;

    console.log("\n");
    console.log(chalk.yellow("ID:"), container.id);
    console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
    console.log(chalk.yellow("CreatedAt"), container.createdAt);
    console.log(chalk.yellow("Uptime"), `${container.uptime} minutes`);
    console.log(chalk.yellow("Version"), container.version);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
