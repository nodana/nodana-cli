import request from "../request";

const getAuthorizationHeader = (key: string) => {
  const delimeter = ".";
  if (!key || !key.includes(delimeter)) return {};

  const [username, password] = key.split(delimeter);

  return {
    Authorization: `Basic ${btoa(`${username}:${password}`)}`,
  };
};

export const exchange = async (token: string = "") => {
  try {
    return request("/keys", "POST", {}, { token });
  } catch (e: any) {
    throw e;
  }
};

export const start = async (key: string, options: any) => {
  try {
    return request("/containers", "POST", getAuthorizationHeader(key), options);
  } catch (e: any) {
    throw e;
  }
};

export const stop = async () => {};

export const list = async (key: string) => {
  try {
    return request("/containers", "GET", getAuthorizationHeader(key));
  } catch (e: any) {
    throw e;
  }
};
