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
  accept?: boolean;
};

export default async (options: Props) => {
  try {
    const confirmed =
      !!options.accept ||
      (await promptly.confirm(
        chalk.yellow(
          "By continuing, you confirm you have read and accept our Terms (https://nodana.io/terms). Continue?[y/n]"
        )
      ));

    if (confirmed) {
      info("Creating container...");
      const response = await client.create(options);

      print(response);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  console.log("\n");
  success(`Container created and started 🚀`);
  console.log("\n");
  console.log(chalk.yellow("ID:"), data.id);
  console.log(chalk.yellow("Connection Url:"), data.connectionUrl);
  data.password && console.log(chalk.yellow("Password"), data.password);
  data.seed && console.log(chalk.yellow("Seed"), data.seed);
  data.webhookSecret &&
    console.log(chalk.yellow("Webhook Secret"), data.webhookSecret);
  console.log("\n");
};
