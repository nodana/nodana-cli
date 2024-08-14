const chalk = require("chalk");
import * as client from "../../client";
import { error, info, success } from "../../helpers/output";
import { getDurationString } from "../../helpers/date";

export default async () => {
  try {
    const services = await client.listServices();

    if (services.length === 0) {
      info("No services found");
      return;
    }

    print(services);
  } catch (e: any) {
    error(e.message);
  }
};

const print = (services: any) => {
  success(`${services.length} service(s) found:`);
  console.log("\n");

  services.forEach((service: any) => {
    Object.keys(service).map((key: string) => {
      const value =
        key === "age"
          ? getDurationString(service.age)
          : key === "fee"
          ? `${service.fee.toLocaleString()} sats`
          : service[key];

      const capitalizedKey = key.charAt(0).toUpperCase() + key.slice(1);
      console.log(`${chalk.yellow(capitalizedKey)}: ${value}`);
    });

    console.log("\n");
  });
};
