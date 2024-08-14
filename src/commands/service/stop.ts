import * as client from "../../client";
import { error, info } from "../../helpers/output";

export default async (id: string) => {
  try {
    info(`Stopping service...`);
    await client.stopService(id);

    print();
  } catch (e: any) {
    error(e.message);
  }
};

const print = () => {
  info(`Service stopped`);
};
