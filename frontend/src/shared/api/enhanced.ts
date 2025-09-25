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

    // favoriting/unfavorting are less critical operations and so we can actually
    // use optimistic update for both of them;
    // https://redux-toolkit.js.org/rtk-query/usage/manual-cache-updates#optimistic-updates
    favoriteArticle: {
      async onQueryStarted({ slug }, { dispatch, queryFulfilled }) {
        const patchResult = dispatch(
          api.util.updateQueryData("readArticle", { slug }, (draft) => {
            Object.assign(draft.article, { favorited: true, favoritesCount: draft.article.favoritesCount + 1 });
          }),
        );
        try {
          await queryFulfilled;
        } catch {
          patchResult.undo();
        }
      },
    },
    unfavoriteArticle: {
      async onQueryStarted({ slug }, { dispatch, queryFulfilled }) {
        const patchResult = dispatch(
          api.util.updateQueryData("readArticle", { slug }, (draft) => {
            Object.assign(draft.article, { favorited: false, favoritesCount: Math.max(0, draft.article.favoritesCount - 1) });
          }),
        );
        try {
          await queryFulfilled;
        } catch {
          patchResult.undo();
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
  useFavoriteArticleMutation,
  useUnfavoriteArticleMutation,
} = api;

export type {
  ConfirmEmailApiArg,
  LoginApiArg,
  UpdateCurrentUserApiArg,
  ArticlePayloadArticle,
  UpdateArticleApiArg,
  UserPayloadUser,
} from "./generated";
