import { BaseQueryApi, FetchArgs, createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

import { config } from "@/config";

import { AppState } from "../types/store.types";

const isDev = __ENV__ === "development";

const baseQuery = fetchBaseQuery({
  baseUrl: config.BACKEND_URL,
  prepareHeaders: (headers, { getState }) => {
    const state = getState() as AppState;

    if (state.auth.isAuthenticated) {
      headers.set("Authorization", `Bearer ${state.auth.user!.token}`);
    }
    return headers;
  },
});

const BaseQueryWrapper: typeof baseQuery = async (args: string | FetchArgs, api: BaseQueryApi, extraOptions) => {
  if (isDev) {
    await new Promise((resolve) => setTimeout(resolve, 700));
  }

  return await baseQuery(args, api, extraOptions);
};

export const base = createApi({
  reducerPath: "api",
  baseQuery: BaseQueryWrapper,
  endpoints: () => ({}),
  tagTypes: ["Article", "ArticleComments", "ArticleGlobalFeed", "ArticlePersonalFeed"],
});
