import { request } from "../request";
import { readFile, writeFile } from "../helpers/file";
import { error } from "../helpers/output";
import { KEY_DELIMITER } from "../constants";

export const getAuthKey = async () => {
  try {
    const key = await readFile();
    return key;
  } catch (e: any) {
    throw new Error("API key not found - have you run init?");
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
    throw new Error("API key could not be saved");
  }
};

export const createService = async (service: string, config: any) => {
  try {
    const key = await getAuthKey();

    return request._call("/containers", "POST", getAuthHeader(key), {
      service,
      config,
    });
  } catch (e: any) {
    throw e;
  }
};

export const startService = async (id: string) => {
  try {
    const key = await getAuthKey();

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

export const stopService = async (id: string) => {
  try {
    const key = await getAuthKey();

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

export const updateService = async (id: string) => {
  try {
    const key = await getAuthKey();

    return request._call(`/containers/${id}`, "POST", getAuthHeader(key), {});
  } catch (e: any) {
    throw e;
  }
};

export const listServices = async () => {
  try {
    const key = await getAuthKey();

    return request._call("/containers", "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const deleteService = async (id: string) => {
  try {
    const key = await getAuthKey();

    return request._call(`/containers/${id}`, "DELETE", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const createInvoice = async (config: any) => {
  try {
    const key = await getAuthKey();

    return request._call("/invoices", "POST", getAuthHeader(key), config);
  } catch (e: any) {
    throw e;
  }
};

export const status = async () => {
  try {
    const key = await getAuthKey();
    const [id] = key.split(KEY_DELIMITER);

    return request._call(`/keys/${id}`, "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};
