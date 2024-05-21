import * as client from "../client";
import { error, success } from "../helpers/output";
import { write as writeFile } from "../helpers/file";
import { CONF_FILE_NAME } from "../constants";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    const response = await client.exchange(token);

    await writeFile(response.key);

    success(`Api key created and saved to ${CONF_FILE_NAME}`);
  } catch (e: any) {
    error(e.message);
  }
};
