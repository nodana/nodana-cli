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
    const apiKey = key || (await readFile());

    const confirmed = await promptly.confirm(
      chalk.yellow("Are you sure?[y/n]")
    );

    if (confirmed) {
      info(`Deleting. Please wait...`);
      const result = await client.stop(apiKey, id);
      if (result.deleted) {
        console.log("\n");
        info(`Container ${id} has been deleted`);
        console.log("\n");
      }
    }
  } catch (e: any) {
    error(e.message);
  }
};
