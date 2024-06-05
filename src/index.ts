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
  .option("-p, --password <string>", "Phoenixd password")
  .option("-s, --seed <string>", "Phoenixd seed")
  .option(
    "-a, --autoLiquidity <string>",
    "Phoenixd auto liquidity value (2m, 5m, 10m)"
  )
  .option("-w, --webhook <string>", "Phoenixd webhook url")
  .option("-x, --webhookSecret <string>", "Phoenixd webhook secret")
  .option(
    "-l, --link",
    "Accept paylink payments. Nodana will need to store your password."
  )
  .option("-y, --yes", "Skip confirmation")
  .action(commands.createNode);

create
  .command("link")
  .description("create a Pay Link")
  .requiredOption("-n, --node <string>", "Node ID")
  .requiredOption("-p, --price <number>", "Pay Link price")
  .requiredOption("-d, --desc <string>", "Pay Link description")
  .option(
    "-c, --currency <string>",
    "Pay Link currency - sat(default), usd, gbp or eur"
  )
  .option("-r, --redirectUrl <string>", "Redirect url after successful payment")
  .option(
    "-e, --email <string>",
    "Email address to send notification of succesfull payment to"
  )
  .action(commands.createLink);

program
  .command("start")
  .description("start a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .action(commands.start);

program
  .command("stop")
  .description("stop a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .action(commands.stop);

program
  .command("delete")
  .description("delete a node")
  .argument("<id>", "Id of node")
  .option("-k, --key <string>", "API Key")
  .option("-y, --accept", "Skip confirmation")
  .action(commands.del);

// Create sub commands for list
const list = program.command("list").description("list Nodana resources");

list
  .command("nodes", { isDefault: true })
  .description("list nodes")
  .option("-k, --key <string>", "API Key")
  .action(commands.listNodes);

list
  .command("links")
  .description("list links")
  .option("-k, --key <string>", "API Key")
  .action(commands.listLinks);

program.parse();
