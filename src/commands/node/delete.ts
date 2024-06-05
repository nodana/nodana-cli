const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { error, info } from "../../helpers/output";

type Options = {
  key?: string;
  accept?: boolean;
};

export default async (id: string, options: Options) => {
  try {
    const confirmed =
      !!options.accept ||
      (await promptly.confirm(
        chalk.yellow("You are about to delete your node. Are you sure?[y/n]")
      ));

    if (confirmed) {
      info(`Deleting. Please wait...`);
      await client.del(id, options);

      print();
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = () => {
  info(`Container deleted`);
};
