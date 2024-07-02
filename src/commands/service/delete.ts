const chalk = require("chalk");
import promptly from "promptly";
import * as client from "../../client";
import { success, error, info } from "../../helpers/output";

type Options = {
  key?: string;
  accept?: boolean;
};

export default async (id: string, options: Options) => {
  try {
    const confirmed =
      !!options.accept ||
      (await promptly.confirm(
        chalk.yellow("You are about to delete your service. Are you sure?[y/n]")
      ));

    if (confirmed) {
      info(`Deleting service...`);
      await client.deleteService(id, options);

      print();
    }
  } catch (e: any) {
    error(e.message);
  }
};

const print = () => {
  success("Service deleted");
};
