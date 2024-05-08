#! /usr/bin/env node

import { Command } from "commander";
// const { Command } = require("commander");

const program = new Command();

const handleCreate = (password: string) => {
  console.log(password);
};

program
  .name("nodana-cli")
  .version("0.0.1")
  .description("A CLI for running Phonenixd instances");

program
  .command("create")
  .description("create a container")
  .argument("<password>", "password")
  .action(handleCreate);

program.parse();
