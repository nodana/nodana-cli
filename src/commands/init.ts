const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../client";
import { error, success } from "../helpers/output";

type Props = {
  accept?: boolean;
};

export default async (options: Props) => {
  try {
    const confirmed =
      !!options.accept ||
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
    error(e.message);
  }
};

const print = () => {
  success(`All done. You can now start creating containers.`);
};
