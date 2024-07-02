const chalk = require("chalk");
import * as client from "../../client";
import { error, info } from "../../helpers/output";

type Options = {
  key?: string;
};

export default async (id: string, options: Options) => {
  try {
    info(`Stopping service...`);
    await client.stopService(id, options);

    print();
  } catch (e: any) {
    error(e.message);
  }
};

const print = () => {
  info(`Service stopped`);
};
