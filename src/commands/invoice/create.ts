const chalk = require("chalk");
import * as client from "../../client";
import { error, info } from "../../helpers/output";

type Props = {
  sats?: number;
};

export default async (options: Props) => {
  try {
    info("Creating invoice...");

    if (!options.sats) {
      throw new Error("Please provide a sats amount (-s)");
    }

    const config = {
      sats: options.sats,
    };

    const response = await client.createInvoice(config);

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (invoice: any) => {
  console.log(`${chalk.green("Invoice created:")} ${invoice.url}`);
  info("The invoice will expire in 1 hour.");
};
