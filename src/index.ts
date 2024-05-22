#! /usr/bin/env node
import { Command } from "commander";
// import figlet from "figlet";
import * as commands from "./commands";

const program = new Command();

// console.log(figlet.textSync("nodana cli"));

program.name("nodana-cli").version("0.0.1").description("Nodana Command Line");

// Exchange token for API key
program
  .command("exchange")
  .description("Exchange token for API key")
  .option("-t, --token <string>", "Token")
  .action(commands.exchange);

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
  .option("-x, --webhookSecret <string>", "Phoenixd webhook secret")
  .action(commands.create);

program
  .command("start")
  .description("Start a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(commands.start);

program
  .command("stop")
  .description("Stop a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(commands.stop);

program
  .command("delete")
  .description("Delete a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(commands.del);

program
  .command("list")
  .description("List running containers")
  .option("-k, --key <string>", "API Key")
  .action(commands.list);

program.parse();
