import path from "path";
const fs = require("fs").promises;
import { CONF_FILE_NAME } from "../../constants";

const getConfigFilepath = () =>
  path.join(__dirname, "..", "..", "..", "..", CONF_FILE_NAME);

export const readFile = async (filepath?: string) => {
  try {
    const contents = await fs.readFile(
      filepath || getConfigFilepath(),
      "utf-8"
    );

    return contents.toString();
  } catch (e: any) {
    throw e;
  }
};

export const writeFile = async (content: string) => {
  try {
    return fs.writeFile(getConfigFilepath(), content);
  } catch (e) {
    throw e;
  }
};

export const fileExists = async () => {
  try {
    await fs.readFile(getConfigFilepath(), "utf-8");
    return true;
  } catch (e) {
    return false;
  }
};

export const deleteFile = async () => {
  try {
    return fs.unlink(getConfigFilepath());
  } catch (e) {
    throw e;
  }
};
