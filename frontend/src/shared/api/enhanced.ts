import { api as generatedApi } from "./generated";

/**
 * Optionallu enhance endpoints.
 *
 * This indirection needed to enhance endpoints with tags and
 * optimistic/pessimistic updates.
 *
 * See: https://redux-toolkit.js.org/rtk-query/api/created-api/code-splitting#enhanceendpoints
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
  },
});

export const {
  useListArticlesQuery,
  useCreateArticleMutation,
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
  UserPayloadUser,
} from "./generated";
