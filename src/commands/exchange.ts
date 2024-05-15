const chalk = require("chalk");
import * as client from "../client";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    const response = await client.exchange(token);

    console.log(chalk.green("API KEY:", response.key));
  } catch (e: any) {
    console.log(e.message);
  }
};
