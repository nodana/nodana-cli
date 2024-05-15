import * as client from "../client";

type Props = {
  key: string;
  password: string;
  seed: string;
  autoLiquidity: string;
  webhook: string;
};

export default async ({ key, ...options }: Props) => {
  try {
    const container = await client.start(key, options);
    console.log(container);
  } catch (e: any) {
    console.log(e.message);
  }
};
