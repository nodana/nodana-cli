#! /usr/bin/env node
import { Command } from "commander";
// import figlet from "figlet";
import { create, del, exchange, list } from "./commands";

const program = new Command();

// console.log(figlet.textSync("nodana cli"));

program.name("nodana-cli").version("0.0.1").description("Nodana Command Line");

// Exchange token for API key
program
  .command("exchange")
  .description("Exchange token for API key")
  .option("-t, --token <string>", "Token")
  .action(exchange);

program
  .command("create")
  .description("Create a container")
  .option("-k, --key <string>", "API Key")
  .option("-p, --password <string>", "Phoenixd password")
  .option("-s, --seed <string>", "Phoenixd seed")
  .option(
    "-a, --autoLiquidity <string>",
    "Phoenixd auto liquidity value (2m, 5m, 10m)"
  )
  .option("-w, --webhook <string>", "Phoenixd webhook url")
  .action(create);

program
  .command("delete")
  .description("Delete a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(del);

// List Containers
program
  .command("list")
  .description("List running containers")
  .option("-k, --key <string>", "API Key")
  .action(list);

program.parse();
