const chalk = require("chalk");
import * as client from "../../client";
import { MIN_INVOICE_VALUE, MAX_INVOICE_VALUE } from "../../constants";
import { error, info, success } from "../../helpers/output";

type Props = {
  key?: string;
  value?: number;
};

export default async (options: Props) => {
  try {
    if (
      !options.value ||
      options.value < MIN_INVOICE_VALUE ||
      options.value > MAX_INVOICE_VALUE
    ) {
      info(
        `Value must be between ${MIN_INVOICE_VALUE} and ${MAX_INVOICE_VALUE}`
      );
      return;
    }

    info("Creating invoice...");
    const response = await client.createInvoice(options);

    print(response);
  } catch (e: any) {
    error("Invoice command could not be completed");
  }
};

const print = (invoice: any) => {
  console.log("\n");
  success(`Invoice created. The invoice will expire in 1 hour.`);
  console.log(chalk.yellow("Payment URL:"), invoice.url);
  console.log(chalk.yellow("ID:"), invoice.id);
  console.log(chalk.yellow("Sats:"), invoice.value);
  invoice.memo && console.log(chalk.yellow("Memo:"), invoice.memo);
};
