import * as client from "../client";
import { error, success } from "../helpers/output";

type Props = {
  key: string;
};

export default async ({ key }: Props) => {
  try {
    const containers = await client.list(key);

    if (containers.length === 0) {
      return success("No containers running");
    }

    const [container] = containers;

    success(`ID: ${container.id},  Created At: ${container.createdAt}`);
  } catch (e: any) {
    error(e.message);
  }
};
