const fs = require("fs").promises;
import { CONF_FILE_NAME } from "../../constants";

const getCwd = () => {
  return process.cwd();
};

const getFilepath = () => {
  return `${getCwd()}/${CONF_FILE_NAME}`;
};

export const read = async () => {
  try {
    const contents = await fs.readFile(getFilepath(), "utf-8");

    return contents.toString();
  } catch (e) {
    throw e;
  }
};

export const write = async (content: string) => {
  try {
    return fs.writeFile(getFilepath(), content);
  } catch (e) {
    throw e;
  }
};