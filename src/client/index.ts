import { request } from "../request";
import { readFile, writeFile } from "../helpers/file";
import { info } from "../helpers/output";
import { KEY_DELIMITER } from "../constants";

export const getAuthKey = async (options: any) => {
  if ("key" in options) return options.key;

  try {
    const key = await readFile();
    return key;
  } catch (e: any) {
    info("Config file not found");
    throw e;
  }
};

export const getAuthHeader = (key: string) => {
  const delimeter = ".";
  if (!key || !key.includes(delimeter)) return {};

  const [username, password] = key.split(delimeter);

  return {
    Authorization: `Basic ${btoa(`${username}:${password}`)}`,
  };
};

export const init = async () => {
  try {
    const response = await request._call("/keys", "POST", {}, {});
    await writeFile(response.key);

    return response;
  } catch (e: any) {
    throw e;
  }
};

export const createNode = async (options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call("/containers", "POST", getAuthHeader(key), options);
  } catch (e: any) {
    throw e;
  }
};

export const startNode = async (id: string, options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call(
      `/containers/${id}/start`,
      "POST",
      getAuthHeader(key),
      {}
    );
  } catch (e: any) {
    throw e;
  }
};

export const stopNode = async (id: string, options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call(
      `/containers/${id}/stop`,
      "POST",
      getAuthHeader(key),
      {}
    );
  } catch (e: any) {
    throw e;
  }
};

export const listNodes = async (options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call("/containers", "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const deleteNode = async (id: string, options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call(`/containers/${id}`, "DELETE", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const createInvoice = async (options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call("/invoices", "POST", getAuthHeader(key), options);
  } catch (e: any) {
    throw e;
  }
};

export const status = async (options: any) => {
  const key = await getAuthKey(options);
  const [id] = key.split(KEY_DELIMITER);

  try {
    return request._call(`/keys/${id}`, "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};
