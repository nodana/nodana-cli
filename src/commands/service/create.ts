const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { error, info, success } from "../../helpers/output";

type Props = {
  service: string;
  options: any;
  key?: string;
  yes?: boolean;
};

export default async (service: string, options: any) => {
  try {
    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow(
          `You are about to create a ${service} service. Are you sure?[y/n]`
        )
      ));

    if (confirmed) {
      info(`Creating ${service} service...`);
      const response = await client.createService(service, options);

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
