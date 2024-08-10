import { API_BASE_URL } from "../constants";

type Headers = {
  [key: string]: any;
};

type Body = {
  [key: string]: any;
};

export const request = {
  get: async function (path: string, headers: Headers) {
    return this._call(path, "GET", headers);
  },

  post: async function (path: string, headers: Headers, body: Body) {
    return this._call(path, "POST", headers, body);
  },

  del: async function (path: string, headers: Headers) {
    return this._call(path, "DELETE", headers, { del: "1" });
  },

  _call: async (
    path: string,
    method: string,
    extraHeaders: any,
    data?: any
  ) => {
    try {
      const options: any = {
        method,
        headers: {
          "Content-Type": "application/json",
          ...extraHeaders,
        },
      };

      if (data) {
        options.body = JSON.stringify(data);
      }

      const response = await fetch(`${API_BASE_URL}${path}`, options);
      const json = await response.json();

      if (!response.ok) {
        throw new Error(json.message);
      }

      return json;
    } catch (e: any) {
      throw e;
    }
  },
};
