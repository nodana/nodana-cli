const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, success, info } from "../helpers/output";
import { fileExists } from "../helpers/file";

type Props = {
  yes?: boolean;
};

export default async (options: Props) => {
  try {
    const configFileExists = await fileExists();
    if (configFileExists) {
      info(
        "API key already exists. You can use 'nodana exit' to remove this key."
      );
      return;
    }

    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow(
          "By continuing, you confirm you have read and accept our terms and conditions (https://nodana.io/terms). Continue?[y/n]"
        )
      ));

    if (confirmed) {
      info("Creating API Key...");
      const response = await client.init();

      print(response.key);
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = (key: string) => {
  info("Creating API Key...");
  console.log("\n");
  console.log(chalk.yellow("API Key: "), key);
  info("Please make a backup of this key.");

  console.log("\n");
  console.log(
    chalk.yellow("Create a service using: "),
    "nodana service create <service-name>"
  );
};
