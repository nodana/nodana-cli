import { API_BASE_URL } from "../constants";

export default async (
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
};
