import { request } from "../request";
import { readFile, writeFile } from "../helpers/file";
import { info } from "../helpers/output";
import { KEY_DELIMITER } from "../constants";

export const getAuthKey = async () => {
  try {
    const key = await readFile();
    return key;
  } catch (e: any) {
    info("nodana.conf could not be found");
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

export const createService = async (service: string, config: any) => {
  const key = await getAuthKey();

  try {
    return request._call("/containers", "POST", getAuthHeader(key), {
      service,
      config,
    });
  } catch (e: any) {
    throw e;
  }
};

export const startService = async (id: string) => {
  const key = await getAuthKey();

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

export const stopService = async (id: string) => {
  const key = await getAuthKey();

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

export const listServices = async () => {
  const key = await getAuthKey();

  try {
    return request._call("/containers", "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const deleteService = async (id: string) => {
  const key = await getAuthKey();

  try {
    return request._call(`/containers/${id}`, "DELETE", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};

export const createInvoice = async (config: any) => {
  const key = await getAuthKey();

  try {
    return request._call("/invoices", "POST", getAuthHeader(key), config);
  } catch (e: any) {
    throw e;
  }
};

export const status = async () => {
  const key = await getAuthKey();
  const [id] = key.split(KEY_DELIMITER);

  try {
    return request._call(`/keys/${id}`, "GET", getAuthHeader(key));
  } catch (e: any) {
    throw e;
  }
};
