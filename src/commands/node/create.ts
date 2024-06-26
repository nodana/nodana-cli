const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { error, info, success } from "../../helpers/output";

type Props = {
  key?: string;
  autoLiquidity?: string;
  webhook?: string;
  yes?: boolean;
};

export default async (options: Props) => {
  try {
    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow("You are about to create a node. Are you sure?[y/n]")
      ));

    if (confirmed) {
      info("Creating node...");
      const response = await client.createNode(options);

      print(response);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  console.log("\n");
  success(`Node created and started 🚀`);
  console.log("\n");
  console.log(chalk.yellow("ID:"), data.id);
  console.log(chalk.yellow("Connection Url:"), data.connectionUrl);
  data.password && console.log(chalk.yellow("Password"), data.password);
  data.seed && console.log(chalk.yellow("Seed"), data.seed);
  data.webhookSecret &&
    console.log(chalk.yellow("Webhook Secret"), data.webhookSecret);
};
