#! /usr/bin/env node
import { Command } from "commander";
import * as commands from "./commands";

const program = new Command();

program.name("nodana-cli").version("0.2.2").description("Nodana CLI");

program
  .command("init")
  .description("create an API key")
  .option(
    "-y, --accept",
    "Accept Nodana's terms and conditions (https://nodana.io/terms)"
  )
  .action(commands.init);

program
  .command("create")
  .description("create a container")
  .option("-k, --key <string>", "API Key")
  .option("-p, --password <string>", "Phoenixd password")
  .option("-s, --seed <string>", "Phoenixd seed")
  .option(
    "-a, --autoLiquidity <string>",
    "Phoenixd auto liquidity value (2m, 5m, 10m)"
  )
  .option("-w, --webhook <string>", "Phoenixd webhook url")
  .option("-x, --webhookSecret <string>", "Phoenixd webhook secret")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.create);

program
  .command("start")
  .description("start a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(commands.start);

program
  .command("stop")
  .description("stop a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .action(commands.stop);

program
  .command("delete")
  .description("delete a container")
  .argument("<id>", "Id of container")
  .option("-k, --key <string>", "API Key")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.del);

program
  .command("list")
  .description("list containers")
  .option("-k, --key <string>", "API Key")
  .action(commands.list);

program.parse();
