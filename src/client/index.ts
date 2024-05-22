import { request } from "../request";
import { read as readFile, write as writeFile } from "../helpers/file";
import { CONF_FILE_NAME } from "../constants";

export const getAuthKey = async (options: any) => {
  if ("key" in options) return options.key;

  try {
    const key = await readFile();
    return key;
  } catch (e: any) {
    throw new Error(`${CONF_FILE_NAME} not found`);
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

export const exchange = async (token: string = "") => {
  try {
    const response = await request._call("/keys", "POST", {}, { token });
    await writeFile(response.key);

    return response;
  } catch (e: any) {
    throw e;
  }
};

export const create = async (options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call("/containers", "POST", getAuthHeader(key), options);
  } catch (e: any) {
    throw e;
  }
};

export const start = async (id: string, options: any) => {
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

export const stop = async (id: string, options: any) => {
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

export const list = async (options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call("/containers", "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const del = async (id: string, options: any) => {
  const key = await getAuthKey(options);

  try {
    return request._call(`/containers/${id}`, "DELETE", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};
