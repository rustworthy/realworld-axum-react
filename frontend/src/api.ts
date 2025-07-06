import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

export type Article = {
  id: number;
  title: string;
  body: string;
  userId: number;
};

export const api = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({
    baseUrl: "https://jsonplaceholder.typicode.com",
  }),
  endpoints: (builder) => ({
    listArticlesByAuthor: builder.query<Article[], number>({
      query: (authorId) => `/posts?userId=${authorId}`,
    }),
  }),
});

export const { useListArticlesByAuthorQuery, useLazyListArticlesByAuthorQuery } = api;
