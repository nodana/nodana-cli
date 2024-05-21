const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info, success } from "../helpers/output";
import { sleep } from "../helpers/date";
import { read as readFile } from "../helpers/file";

type Props = {
  key: string;
  password: string;
  seed: string;
  autoLiquidity: string;
  webhook: string;
};

export default async ({ key, ...options }: Props) => {
  try {
    const apiKey = key || (await readFile());

    const confirmed = await promptly.confirm(
      chalk.yellow("Are you sure?[y/n]")
    );

    if (confirmed) {
      info("Creating container...");
      await sleep(1000);
      const container = await client.start(apiKey, options);

      console.log("\n");
      success(`Container is up and running 🚀`);
      console.log("\n");
      console.log(chalk.yellow("ID:"), container.id);
      console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
      container.password &&
        console.log(chalk.yellow("Password"), container.password);
      container.seed && console.log(chalk.yellow("Seed"), container.seed);
      console.log("\n");
    }
  } catch (e: any) {
    error(e.message);
  }
};
