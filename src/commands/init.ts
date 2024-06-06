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
        "Config file already exists. You can use 'nodana exit' to remove the config file."
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
      await client.init();

      print();
    }
  } catch (e: any) {
    error("Init command could not be completed");
  }
};

const print = () => {
  success(`All done. You can now start creating containers.`);
};
