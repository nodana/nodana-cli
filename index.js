#! /usr/bin/env node

const { program } = require("commander");
const createContainer = require("./commands/containers/create");

program
  .command("create")
  .description("Create a container with Phoenixd installed")
  .action(createContainer);

program.parse();
