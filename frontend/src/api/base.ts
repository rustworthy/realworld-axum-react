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
  // ONLY FOR INITIAL SETUP PURPOSES, WILL BE REMOVED ONCE OPENAPI
  // SPEC IN MADE AVAILABLE FOR CODE GENERATION
  endpoints: (builder) => ({
    listArticlesByAuthor: builder.query<Article[], number>({
      query: (authorId) => `/posts?userId=${authorId}`,
    }),
  }),
});

export const { useListArticlesByAuthorQuery, useLazyListArticlesByAuthorQuery } = base;
