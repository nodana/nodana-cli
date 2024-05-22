const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info, success } from "../helpers/output";

type Options = {
  key?: string;
};

export default async (id: string, options: Options) => {
  try {
    const confirmed = await promptly.confirm(
      chalk.yellow("Are you sure?[y/n]")
    );

    if (confirmed) {
      info(`Deleting. Please wait...`);
      const result = await client.del(id, options);
      success(`Container deleted ❌`);
      console.log("\n");
      console.log(chalk.yellow("Fee:"), `${result.fee} sats`);
      console.log(
        chalk.yellow("Sats Remaining:"),
        `${result.satsRemaining} sats`
      );
      console.log("\n");
    }
  } catch (e: any) {
    error(e.message);
  }
};
