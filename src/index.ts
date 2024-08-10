#! /usr/bin/env node
import { Command } from "commander";
import * as commands from "./commands";

const program = new Command();

program
  .name("nodana-cli")
  .version("0.5.3", "-v, --version", "output the current version")
  .description("Nodana CLI");

program
  .command("init")
  .description("create an API key")
  .option(
    "-y, --yes",
    "Accept Nodana's Terms and Conditions (https://nodana.io/terms)"
  )
  .action(commands.init);

program
  .command("status")
  .description("Check status of API")
  .option("-k, --key <string>", "API Key")
  .action(commands.status);

program
  .command("exit")
  .description("removes an API key")
  .option("-y, --yes", "Skip confirmation")
  .action(commands.exit);

// Create sub commands for create
const service = program.command("service");

service
  .command("start")
  .description("start a service")
  .argument("<id>", "Id of service")
  .option("-k, --key <string>", "API Key")
  .action(commands.startService);

service
  .command("stop")
  .description("stop a service")
  .argument("<id>", "Id of service")
  .option("-k, --key <string>", "API Key")
  .action(commands.stopService);

service
  .command("list")
  .description("list services")
  .option("-k, --key <string>", "API Key")
  .action(commands.listServices);

service
  .command("delete")
  .description("delete a service")
  .argument("<id>", "Id of service")
  .option("-k, --key <string>", "API Key")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.deleteService);

const create = service.command("create").description("create a service");

create
  .command("phoenixd")
  .option("-k, --key <string>", "API Key (overrides config file key)")
  .option("-n, --name <string>", "Identifier")
  .option("-a, --autoLiquidity <string>", "Auto liquidity value (2m, 5m, 10m)")
  .option("-s, --seed <string>", "12 word seed phrase to restore wallet")
  .option("-w, --webhook <string>", "Webhook url")
  .option("-y, --yes", "Skip confirmation")
  .action((options: any) => {
    commands.createService("phoenixd", options);
  });

create
  .command("albyhub")
  .option("-k, --key <string>", "API Key (overrides config file key)")
  .option("-n, --name <string>", "Identifier")
  .option("-y, --yes", "Skip confirmation")
  .action((options: any) => {
    commands.createService("albyhub", options);
  });

const invoice = program.command("invoice");

invoice
  .command("create")
  .description("create an invoice")
  .option("-k, --key <string>", "API Key")
  .requiredOption(
    "-s, --sats <string>",
    "Invoice value in sats (min: 1k, max: 1m)"
  )
  .action(commands.createInvoice);

program.parse();
