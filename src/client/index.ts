import { request } from "../request";

export const getAuthorizationHeader = (key: string) => {
  const delimeter = ".";
  if (!key || !key.includes(delimeter)) return {};

  const [username, password] = key.split(delimeter);

  return {
    Authorization: `Basic ${btoa(`${username}:${password}`)}`,
  };
};

export const exchange = async (token: string = "") => {
  try {
    return request._call("/keys", "POST", {}, { token });
  } catch (e: any) {
    throw e;
  }
};

export const create = async (key: string, options: any) => {
  try {
    return request._call(
      "/containers",
      "POST",
      getAuthorizationHeader(key),
      options
    );
  } catch (e: any) {
    throw e;
  }
};

export const del = async (key: string, id: string) => {
  try {
    return request._call(
      `/containers/${id}`,
      "DELETE",
      getAuthorizationHeader(key)
    );
  } catch (e: any) {
    throw e;
  }
};

export const list = async (key: string) => {
  try {
    return request._call("/containers", "GET", getAuthorizationHeader(key));
  } catch (e: any) {
    throw e;
  }
};
