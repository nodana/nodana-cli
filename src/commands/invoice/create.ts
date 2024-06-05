const chalk = require("chalk");
import * as client from "../../client";
import { MIN_INVOICE_VALUE, MAX_INVOICE_VALUE } from "../../constants";
import { error, info, success } from "../../helpers/output";

type Props = {
  key?: string;
  value?: number;
  memo?: string;
};

export default async (options: Props) => {
  try {
    if (
      !options.value ||
      options.value < MIN_INVOICE_VALUE ||
      options.value > MAX_INVOICE_VALUE
    ) {
      throw Error(
        `Value must be between ${MIN_INVOICE_VALUE} and ${MAX_INVOICE_VALUE}`
      );
    }

    info("Creating invoice...");
    const response = await client.createInvoice(options);

    print(response);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (invoice: any) => {
  console.log("\n");
  success(`Invoice created`);
  console.log("\n");
  console.log(chalk.yellow("ID:"), invoice.id);
  console.log(chalk.yellow("Value (sats):"), invoice.value);
  console.log(chalk.yellow("Payment Request:"), invoice.pr);
  invoice.memo && console.log(chalk.yellow("memo:"), invoice.memo);
};
