const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { success, error, info } from "../../helpers/output";

type Options = {
  accept?: boolean;
};

export default async (id: string, options: Options) => {
  try {
    const confirmed =
      !!options.accept ||
      (await promptly.confirm(
        chalk.yellow("You are about to update your service. Are you sure?[y/n]")
      ));

    if (confirmed) {
      info(`Updating service...`);
      const response = await client.updateService(id);

      print(response);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  success(`Service updated`);
  console.log("\n");
  console.log(chalk.yellow("Id:"), data.id);
  console.log(chalk.yellow("Version:"), data.version);
  console.log("\n");
};
