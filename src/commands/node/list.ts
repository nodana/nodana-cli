const chalk = require("chalk");
import * as client from "../../client";
import { error, info } from "../../helpers/output";
import { getDurationString } from "../../helpers/date";

type Props = {
  key?: string;
};

export default async (options: Props) => {
  try {
    const nodes = await client.listNodes(options);

    if (nodes.length === 0) {
      info("No nodes found");
      return;
    }

    print(nodes);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (nodes: any) => {
  nodes.forEach((node: any) => {
    console.log("\n");
    console.log(chalk.yellow("ID:"), node.id);
    node.name && console.log(chalk.yellow("Name:"), node.name);
    console.log(chalk.yellow("Service"), node.service);
    console.log(chalk.yellow("Connection Url:"), node.connectionUrl);
    console.log(chalk.yellow("Age"), getDurationString(node.age));
    console.log(chalk.yellow("Fee"), `${node.fee.toLocaleString()} sats`);
    console.log(chalk.yellow("Status"), node.status);
    console.log(chalk.yellow("Version"), node.version);
  });
};
