import * as client from "../client";
import { error, success } from "../helpers/output";
import { CONF_FILE_NAME } from "../constants";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    await client.exchange(token);
    console.log("\n");
    success(`Api key created and saved to ${CONF_FILE_NAME}`);
    console.log("\n");
  } catch (e: any) {
    error(e.message);
  }
};
