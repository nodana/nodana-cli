const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, info } from "../helpers/output";
import { read as readFile } from "../helpers/file";

type Options = {
  key: string;
};

export default async (id: string, { key }: Options) => {
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
      info(`Deleting. Please wait...`);
      const result = await client.stop(apiKey, id);
      console.log("\n");
      console.log(chalk.yellow("Deleted:"), result.deleted.toString());
      console.log(chalk.yellow("Fee:"), `${result.fee}sats`);
      console.log("\n");
    }
  } catch (e: any) {
    error(e.message);
  }
};
