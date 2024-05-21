import * as client from "../client";
import { error, success } from "../helpers/output";
import { write as writeFile } from "../helpers/file";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    const response = await client.exchange(token);

    await writeFile(response.key);

    success(`API KEY: ${response.key}`);
  } catch (e: any) {
    error(e.message);
  }
};
