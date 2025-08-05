import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { config } from "@/config";

export type Article = {
  id: number;
  title: string;
  body: string;
  userId: number;
};

export const base = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({
    baseUrl: config.BACKEND_URL,
    prepareHeaders: (headers, { getState }) => {
      // effectively no-op for now, but will be updated with the JWT,
      // once the hook is in place
      // @ts-expect-error - it's unknown for the time being
      const token = getState().auth?.user?.token;
      if (token) {
        headers.set("Authorization", `Bearer ${token}`);
      }
      return headers;
    },
  }),
  endpoints: () => ({}),
});
