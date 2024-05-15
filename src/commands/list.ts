import * as client from "../client";

type Props = {
  key: string;
};

export default async ({ key }: Props) => {
  try {
    const containers = await client.list(key);
    console.log(containers);
  } catch (e: any) {
    console.log(e.message);
  }
};
