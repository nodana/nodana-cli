const chalk = require("chalk");
import promptly from "promptly";
import { error, success } from "../helpers/output";
import { deleteFile } from "../helpers/file";

type Props = {
  yes?: boolean;
};

export default async (options: Props) => {
  try {
    const confirmed =
      !!options.yes ||
      (await promptly.confirm(
        chalk.yellow(
          "You are about to delete your config file. Are you sure?[y/n]"
        )
      ));

    if (confirmed) {
      await deleteFile();

      print();
    }
  } catch (e: any) {
    error("Exit command could not be completed");
  }
};

const print = () => {
  success(`Config file deleted.`);
};
