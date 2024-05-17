const chalk = require("chalk");
import * as client from "../client";
import { error, success } from "../helpers/output";

type Props = {
  key: string;
  password: string;
  seed: string;
  autoLiquidity: string;
  webhook: string;
};

export default async ({ key, ...options }: Props) => {
  try {
    const container = await client.start(key, options);
    success(
      `Container ${chalk.bold.underline(container.id)} is up and running 🚀`
    );
  } catch (e: any) {
    error(e.message);
  }
};
