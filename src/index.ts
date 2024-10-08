#! /usr/bin/env node
import { Command } from "commander";
import * as commands from "./commands";

const program = new Command();

program
  .name("nodana-cli")
  .version("0.7.0", "-v, --version", "output the current version")
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
  .action(commands.status);

program
  .command("exit")
  .description("removes an API key")
  .option("-y, --yes", "Skip confirmation")
  .action(commands.exit);

// Create sub commands for create
const service = program.command("service");

service
  .command("create")
  .description("create a service")
  .argument("[service]", "create service without any settings")
  .option("-f, --file <string>", "Path to toml service file")
  .option("-y, --yes", "Skip confirmation")
  .action((service, options: any) => {
    commands.createService(service, options);
  });

service
  .command("start")
  .description("start a service")
  .argument("<id>", "Id of service")
  .action(commands.startService);

service
  .command("stop")
  .description("stop a service")
  .argument("<id>", "Id of service")
  .action(commands.stopService);

service
  .command("update")
  .description("update a service")
  .argument("<id>", "Id of service")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.updateService);

service
  .command("list")
  .description("list services")
  .action(commands.listServices);

service
  .command("delete")
  .description("delete a service")
  .argument("<id>", "Id of service")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.deleteService);

const invoice = program.command("invoice");

invoice
  .command("create")
  .description("create an invoice")
  .requiredOption(
    "-s, --sats <string>",
    "Invoice value in sats (min: 1k, max: 1m)"
  )
  .action(commands.createInvoice);

program.parse();
