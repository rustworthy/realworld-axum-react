import { api } from "./generated";

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
