const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { error, info, success } from "../../helpers/output";

type Props = {
  type: string;
  key?: string;
  yes?: boolean;
};

export default async (options: any) => {
  try {
    const { type } = options;

    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow(
          `You are about to create a ${type} service. Are you sure?[y/n]`
        )
      ));

    if (confirmed) {
      info(`Creating ${type} service...`);
      const response = await client.createService(options);

      print(response);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (data: any) => {
  success(`Service created and started 🚀`);
  console.log("\n");

  Object.keys(data).map((key: string) => {
    const capitalizedKey = key.charAt(0).toUpperCase() + key.slice(1);
    const value = data[key];
    console.log(`${chalk.yellow(capitalizedKey)}: ${value}`);
  });

  console.log("\n");
};
