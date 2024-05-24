const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info } from "../helpers/output";

type Options = {
  key?: string;
  accept?: boolean;
};

export default async (id: string, options: Options) => {
  try {
    const confirmed =
      !!options.accept ||
      (await promptly.confirm(
        chalk.yellow(
          "You are about to delete your container. Are you sure?[y/n]"
        )
      ));

    if (confirmed) {
      info(`Deleting. Please wait...`);
      const response = await client.del(id, options);

      print(response);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  info(`Container deleted ❌`);
  console.log("\n");
  console.log(chalk.yellow("Fee:"), `${data.fee} sats`);
  console.log(chalk.yellow("Sats Remaining:"), `${data.satsRemaining} sats`);
  console.log("\n");
};
