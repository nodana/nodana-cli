import * as client from "../client";
import { error, success } from "../helpers/output";

type Options = {
  key: string;
};

export default async (id: string, { key }: Options) => {
  try {
    const result = await client.stop(key, id);
    if (result.deleted) {
      success(`Container ${id} has been stopped`);
    }
  } catch (e: any) {
    error(e.message);
  }
};
