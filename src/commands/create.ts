const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info, success } from "../helpers/output";
import { sleep } from "../helpers/date";
import { read as readFile } from "../helpers/file";

type Props = {
  key?: string;
  password?: string;
  seed?: string;
  autoLiquidity?: string;
  webhook?: string;
};

export default async ({ key, ...options }: Props) => {
  try {
    let apiKey;

    try {
      apiKey = key || (await readFile());
    } catch (e: any) {
      throw new Error("Nodana config file not found");
    }

    const confirmed = await promptly.confirm(
      chalk.yellow("Are you sure?[y/n]")
    );

    if (confirmed) {
      info("Creating container...");
      // await sleep(1000);
      const container = await client.create(apiKey, options);

      console.log("\n");
      success(`Container is up and running 🚀`);
      console.log("\n");
      console.log(chalk.yellow("ID:"), container.id);
      console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
      console.log("\n");
      info(
        `Please record the following details as they won't be retrievable via the CLI`
      );
      container.password &&
        console.log(chalk.yellow("Password"), container.password);
      container.seed && console.log(chalk.yellow("Seed"), container.seed);
      container.webhookSecret &&
        console.log(chalk.yellow("Webhook Secret"), container.webhookSecret);
      console.log("\n");
    }
  } catch (e: any) {
    error(e.message);
  }
};
