import * as client from "../client";
import { error, success } from "../helpers/output";

type Props = {
  token: string;
};

export default async ({ token }: Props) => {
  try {
    const response = await client.exchange(token);

    success(`API KEY: ${response.key}`);
  } catch (e: any) {
    error(e.message);
  }
};
