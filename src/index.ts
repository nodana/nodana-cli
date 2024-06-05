#! /usr/bin/env node
import { Command } from "commander";
import * as commands from "./commands";

const program = new Command();

program.name("nodana-cli").version("0.3.0").description("Nodana CLI");

program
  .command("init")
  .description("create an API key")
  .option(
    "-y, --yes",
    "Accept Nodana's terms and conditions (https://nodana.io/terms)"
  )
  .action(commands.init);

// Create sub commands for create
const create = program.command("create").description("create Nodana resources");

create
  .command("node", { isDefault: true })
  .description("create a node")
  .option("-k, --key <string>", "API Key")
  .option("-n, --name <string>", "Node name")
  .option("-p, --password <string>", "Node password")
  .option("-s, --seed <string>", "Node seed")
  .option("-a, --autoLiquidity <string>", "Auto liquidity value (2m, 5m, 10m)")
  .option("-w, --webhook <string>", "Webhook url")
  .option("-x, --webhookSecret <string>", "Webhook secret")
  .option("-y, --yes", "Skip confirmation")
  .action(commands.createNode);

program
  .command("start")
  .description("start a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .action(commands.startNode);

program
  .command("stop")
  .description("stop a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .action(commands.stopNode);

program
  .command("delete")
  .description("delete a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.deleteNode);

// Create sub commands for list
const list = program.command("list").description("list Nodana resources");

list
  .command("nodes", { isDefault: true })
  .description("list nodes")
  .option("-k, --key <string>", "API Key")
  .action(commands.listNodes);

program.parse();
