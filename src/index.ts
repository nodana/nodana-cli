#! /usr/bin/env node
import { Command } from "commander";
import { start, exchange, list, stop } from "./commands";

const program = new Command();

program.name("nodana-cli").version("0.0.1").description("Nodana Command Line");

// Exchange token for API key
program
  .command("exchange")
  .description("Exchange token for API key")
  .option("-t, --token <string>", "Token")
  .action(exchange);

// List Containers
program
  .command("list")
  .description("List running containers")
  .requiredOption("-k, --key <string>", "API Key")
  .action(list);

// Create Container
// autoLiquidity
// password
// seed
// webhook
// webhookSecret
program
  .command("start")
  .description("Create and start a container")
  .requiredOption("-k, --key <string>", "API Key")
  .option("-p, --password <string>", "Phoenixd password")
  .option("-s, --seed <string>", "Phoenixd seed")
  .option(
    "-a, --autoLiquidity <string>",
    "Phoenixd auto liquidity value (2m, 5m, 10m)"
  )
  .option("-w, --webhook <string>", "Phoenixd webhook url")
  .action(start);

program
  .command("stop")
  .description("Stop and remove a container")
  .argument("<id>", "Id of container")
  .requiredOption("-k, --key <string>", "API Key")
  .action(stop);

program.parse();
