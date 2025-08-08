import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { config } from "@/config";

import { AppState } from "../providers/ReduxProvider/store";

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
      const state = getState() as AppState;
      if (state.auth.isAuthenticated) {
        headers.set("Authorization", `Bearer ${state.auth.user!.token}`);
      }
      return headers;
    },
  }),
  endpoints: () => ({}),
});
