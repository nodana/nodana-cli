const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info, success } from "../helpers/output";

type Props = {
  key?: string;
  password?: string;
  seed?: string;
  autoLiquidity?: string;
  webhook?: string;
};

export default async (options: Props) => {
  try {
    const confirmed = await promptly.confirm(
      chalk.yellow("Are you sure?[y/n]")
    );

    if (confirmed) {
      info("Creating container...");
      const container = await client.create(options);

      console.log("\n");
      success(`Container created and started 🚀`);
      console.log("\n");
      console.log(chalk.yellow("ID:"), container.id);
      console.log(chalk.yellow("Connection Url:"), container.connectionUrl);
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
