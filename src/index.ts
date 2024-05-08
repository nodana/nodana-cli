#! /usr/bin/env node

const { Command } = require("commander"); // add this line
const program = new Command();

const handleCreate = (str: string, options: any) => {
  console.log(str);
  console.log(options);
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
