import * as client from "../client";

type Options = {
  key: string;
};

export default async (id: string, { key }: Options) => {
  try {
    // if(!id) {
    //   throw new Error('Id must be provided');
    // }
    const result = await client.stop(key, id);
    console.log(result);
  } catch (e: any) {
    console.log(e.message);
  }
};
