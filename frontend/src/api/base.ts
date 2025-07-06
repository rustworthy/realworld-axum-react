import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { config } from "../config";

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
  }),
  endpoints: () => ({}),
});
