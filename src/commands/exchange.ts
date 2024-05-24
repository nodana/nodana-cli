import * as client from "../client";
import { error, success } from "../helpers/output";
import { CONF_FILE_NAME } from "../constants";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    await client.exchange(token);

    print();
  } catch (e: any) {
    error(e.message);
  }
};

const print = () => {
  success(`Api key created and saved to ${CONF_FILE_NAME}`);
};
