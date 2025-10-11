import { AppState } from "../types/store.types";
import { ListArticlesApiArg, api as generatedApi } from "./generated";

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
    listArticles: {
      providesTags: (result, _error, arg) =>
        result ? [{ type: "ArticleGlobalFeed" as const, id: JSON.stringify(arg) }, "ArticleGlobalFeed"] : ["ArticleGlobalFeed"],
    },
    personalFeed: {
      providesTags: (result, _error, arg) =>
        result
          ? [{ type: "ArticlePersonalFeed" as const, id: JSON.stringify(arg) }, "ArticlePersonalFeed"]
          : ["ArticlePersonalFeed"],
    },
    readArticle: {
      providesTags: (result, _error, { slug }) => (result ? [{ type: "Article" as const, id: slug }, "Article"] : ["Article"]),
    },
    createArticle: {
      // TODO: we can be smarter here and prepend this article to
      // 1) their global feed and 2) list of articles by tag; for
      // now though we are going with invalidation
      invalidatesTags: ["ArticleGlobalFeed"],
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
      async onQueryStarted({ slug }, { dispatch, queryFulfilled, getState }) {
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
          // we also need to update this item in the feeds for which we can
          // utilize an rtk's `selectInvalidatedBy` helper that returns all
          // the chache registries this item is found in:
          // https://redux-toolkit.js.org/rtk-query/api/created-api/api-slice-utils#selectinvalidatedby
          for (const { endpointName, originalArgs } of api.util.selectInvalidatedBy(getState(), ["ArticleGlobalFeed"])) {
            if (endpointName !== "listArticles" && endpointName !== "personalFeed") continue;
            dispatch(
              api.util.updateQueryData(endpointName, originalArgs, (draft) => {
                const article = draft.articles.find((article) => article.slug === slug);
                if (article) Object.assign(article, data.article);
              }),
            );
          }
        } catch {
          // no-op
        }
      },
    },
    listComments: {
      providesTags: (result, _error, { slug }) => (result ? [{ type: "ArticleComments" as const, id: slug }] : []),
    },
    createComment: {
      // TODO: update optimistically
      invalidatesTags: (result, _error, { slug }) => (result ? [{ type: "ArticleComments" as const, id: slug }] : []),
    },
    // favoriting/unfavorting are less critical operations and so we can actually
    // use optimistic update for both of them;
    //
    // one caveat with favoriting has to do with the fact that there is a view
    // with the list of favorited articles on the profile page (query includes
    // `favorited=username`), and so we need to either add an article to that cached
    // list if the list exists (which is tricky because of pagination dimension),
    // or refetch the favorited list;
    //
    // https://redux-toolkit.js.org/rtk-query/usage/manual-cache-updates#optimistic-updates
    favoriteArticle: {
      async onQueryStarted({ slug }, { dispatch, queryFulfilled, getState }) {
        const patchResults = [];
        patchResults.push(
          dispatch(
            api.util.updateQueryData("readArticle", { slug }, (draft) => {
              Object.assign(draft.article, { favorited: true, favoritesCount: draft.article.favoritesCount + 1 });
            }),
          ),
        );
        // just like in `updateArticle` operation, search for this article in
        // cache registries and only surgically update it in each registry ...
        const state = getState() as AppState;
        for (const { endpointName, originalArgs } of api.util.selectInvalidatedBy(state, [
          "ArticleGlobalFeed",
          "ArticlePersonalFeed",
        ])) {
          if (endpointName !== "listArticles" && endpointName !== "personalFeed") continue;
          patchResults.push(
            dispatch(
              api.util.updateQueryData(endpointName, originalArgs, (draft) => {
                const cachedArticle = draft.articles.find((article) => article.slug === slug);
                if (cachedArticle) {
                  cachedArticle.favorited = true;
                  cachedArticle.favoritesCount += 1;
                }
              }),
            ),
          );
          // ... and invalidate the favorited list
          if ((originalArgs as ListArticlesApiArg).favorited === state.auth.user?.username) {
            dispatch(api.util.invalidateTags([{ type: "ArticleGlobalFeed", id: JSON.stringify(originalArgs) }]));
          }
        }
        try {
          await queryFulfilled;
        } catch {
          patchResults.forEach((result) => result.undo());
        }
      },
    },
    unfavoriteArticle: {
      async onQueryStarted({ slug }, { dispatch, queryFulfilled, getState }) {
        const patchResults = [];
        patchResults.push(
          dispatch(
            api.util.updateQueryData("readArticle", { slug }, (draft) => {
              Object.assign(draft.article, { favorited: false, favoritesCount: Math.max(0, draft.article.favoritesCount - 1) });
            }),
          ),
        );
        // similar to `favoriteArticle` operation, update the article if it's
        // found in cached lists ...
        const state = getState() as AppState;
        for (const { endpointName, originalArgs } of api.util.selectInvalidatedBy(state, [
          "ArticleGlobalFeed",
          "ArticlePersonalFeed",
        ])) {
          if (endpointName !== "listArticles" && endpointName !== "personalFeed") continue;
          patchResults.push(
            dispatch(
              api.util.updateQueryData(endpointName, originalArgs, (draft) => {
                const article = draft.articles.find((article) => article.slug === slug);
                if (article) {
                  Object.assign(article, { favorited: false, favoritesCount: Math.max(0, article.favoritesCount - 1) });
                }
              }),
            ),
          );
          // ... and invalidate the favorited list
          if ((originalArgs as ListArticlesApiArg).favorited === state.auth.user?.username) {
            dispatch(api.util.invalidateTags([{ type: "ArticleGlobalFeed", id: JSON.stringify(originalArgs) }]));
          }
        }
        try {
          await queryFulfilled;
        } catch {
          patchResults.forEach((result) => result.undo());
        }
      },
    },
    deleteArticle: {
      // we could try and scan the cache entries (namely, feeds) and filter
      // our this item from them, but then we should also be accounting for
      // "broken" pagination in this case; we are keeping things simple and
      // just re-fetch the data from server;
      //
      // note that it seems like we should not be updating the personal feed,
      // since the author cannot follow themselves and so this article should
      // not be in the personal feed and so there is no need to refetch, but
      // then, if we decide that there should be other roles capable of deleting
      // articles this can get messy;
      invalidatesTags: ["ArticleGlobalFeed"],
    },
    followProfile: {
      invalidatesTags: ["ArticlePersonalFeed"],
      async onQueryStarted({ username }, { dispatch, queryFulfilled, getState }) {
        const patchResults = [];
        patchResults.push(
          dispatch(
            api.util.updateQueryData("profile", { username }, (draft) => {
              draft.profile.following = true;
            }),
          ),
        );
        const state = getState() as AppState;
        for (const { endpointName, originalArgs } of api.util.selectInvalidatedBy(state, ["Article"])) {
          if (endpointName !== "readArticle") continue;
          patchResults.push(
            dispatch(
              api.util.updateQueryData(endpointName, originalArgs, (draft) => {
                if (draft.article.author.username === username) {
                  draft.article.author.following = true;
                }
              }),
            ),
          );
        }
        try {
          await queryFulfilled;
        } catch {
          patchResults.forEach((result) => result.undo());
        }
      },
    },
    unfollowProfile: {
      invalidatesTags: ["ArticlePersonalFeed"],
      async onQueryStarted({ username }, { dispatch, queryFulfilled, getState }) {
        const patchResults = [];
        patchResults.push(
          dispatch(
            api.util.updateQueryData("profile", { username }, (draft) => {
              Object.assign(draft.profile, { following: false });
            }),
          ),
        );
        const state = getState() as AppState;
        for (const { endpointName, originalArgs } of api.util.selectInvalidatedBy(state, ["Article"])) {
          if (endpointName !== "readArticle") continue;
          patchResults.push(
            dispatch(
              api.util.updateQueryData(endpointName, originalArgs, (draft) => {
                if (draft.article.author.username === username) {
                  draft.article.author.following = false;
                }
              }),
            ),
          );
        }
        try {
          await queryFulfilled;
        } catch {
          patchResults.forEach((result) => result.undo());
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
  usePersonalFeedQuery,
  useListTagsQuery,
  useProfileQuery,
  useFollowProfileMutation,
  useUnfollowProfileMutation,
  useListCommentsQuery,
  useCreateCommentMutation,
  useDeleteCommentMutation,
} = api;

export type {
  ConfirmEmailApiArg,
  LoginApiArg,
  UpdateCurrentUserApiArg,
  ArticlePayloadArticle,
  ListArticlesApiResponse,
  UpdateArticleApiArg,
  CreateCommentApiArg,
  UserPayloadUser,
} from "./generated";
