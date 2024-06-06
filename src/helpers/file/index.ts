import path from "path";
const fs = require("fs").promises;
import { CONF_FILE_NAME } from "../../constants";

const getFilepath = () =>
  path.join(__dirname, "..", "..", "..", "..", CONF_FILE_NAME);

export const readFile = async () => {
  try {
    const contents = await fs.readFile(getFilepath(), "utf-8");

    return contents.toString();
  } catch (e) {
    throw new Error("Config file could not be found");
  }
};

export const writeFile = async (content: string) => {
  try {
    return fs.writeFile(getFilepath(), content);
  } catch (e) {
    throw new Error("Config file could not be created");
  }
};

export const fileExists = async () => {
  try {
    await fs.readFile(getFilepath(), "utf-8");
    return true;
  } catch (e) {
    return false;
  }
};

export const deleteFile = async () => {
  try {
    return fs.unlink(getFilepath());
  } catch (e) {
    throw new Error("Config file could not be deleted");
  }
};
