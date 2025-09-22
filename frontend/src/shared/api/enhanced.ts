import { api as generatedApi } from "./generated";

/**
 * Optionally enhance endpoints.
 *
 * This indirection needed to enhance endpoints with tags and
 * optimistic/pessimistic updates.
 *
 * @see https://redux-toolkit.js.org/rtk-query/api/created-api/code-splitting#enhanceendpoints
 * @see https://redux-toolkit.js.org/rtk-query/usage/manual-cache-updates
 */
const api = generatedApi.enhanceEndpoints({
  endpoints: {
    createArticle: {
      async onQueryStarted(_, { dispatch, queryFulfilled }) {
        try {
          const { data } = await queryFulfilled;
          dispatch(generatedApi.util.upsertQueryData("readArticle", { slug: data.article.slug }, data));
        } catch {
          // no-op
        }
      },
    },
    updateArticle: {
      async onQueryStarted({ slug }, { dispatch, queryFulfilled }) {
        try {
          const { data } = await queryFulfilled;
          if (data.article.slug === slug) {
            // article's title has not been changed and so we _patch_
            // the existing cache entry
            dispatch(
              api.util.updateQueryData("readArticle", { slug }, (draft) => {
                Object.assign(draft, data);
              }),
            );
          } else {
            // the article's slug has changed (as a result of title change) and
            // so we need to insert a brand new cache entry
            dispatch(generatedApi.util.upsertQueryData("readArticle", { slug: data.article.slug }, data));
          }
        } catch {
          // no-op
        }
      },
    },
  },
});

export const {
  useListArticlesQuery,
  useCreateArticleMutation,
  useUpdateArticleMutation,
  useReadArticleQuery,
  useDeleteArticleMutation,
  useReadCurrentUserQuery,
  useUpdateCurrentUserMutation,
  useRegisterUserMutation,
  useConfirmEmailMutation,
  useLoginMutation,
} = api;

export type {
  ConfirmEmailApiArg,
  LoginApiArg,
  UpdateCurrentUserApiArg,
  ArticlePayloadArticle,
  UpdateArticleApiArg,
  UserPayloadUser,
} from "./generated";
