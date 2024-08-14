const chalk = require("chalk");
import promptly from "promptly";
const tomlParser = require("toml");
import * as client from "../../client";
import { readFile } from "../../helpers/file";
import { error, info, success } from "../../helpers/output";

type Props = {
  options: any;
  yes?: boolean;
};

export default async (options: any) => {
  try {
    let service = "";
    let config: any = {};

    try {
      const toml = await readFile(options.config);
      const json = tomlParser.parse(toml);

      service = json.service;
      config = json.settings || {};
    } catch (e) {
      throw new Error("Service file could not be found");
    }

    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow(
          `You are about to create a ${service} service. Are you sure?[y/n]`
        )
      ));

    if (confirmed) {
      info(`Creating ${service} service...`);
      const response = await client.createService(service, config);

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
