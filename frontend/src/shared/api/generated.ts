import { base as api } from "./base";

const injectedRtkApi = api.injectEndpoints({
  endpoints: (build) => ({
    createArticle: build.mutation<CreateArticleApiResponse, CreateArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles`,
        method: "POST",
        body: queryArg.articlePayloadArticleCreate,
      }),
    }),
    readCurrentUser: build.query<ReadCurrentUserApiResponse, ReadCurrentUserApiArg>({
      query: () => ({ url: `/api/user` }),
    }),
    updateCurrentUser: build.mutation<UpdateCurrentUserApiResponse, UpdateCurrentUserApiArg>({
      query: (queryArg) => ({
        url: `/api/user`,
        method: "PUT",
        body: queryArg.userPayloadUserUpdate,
      }),
    }),
    registerUser: build.mutation<RegisterUserApiResponse, RegisterUserApiArg>({
      query: (queryArg) => ({
        url: `/api/users`,
        method: "POST",
        body: queryArg.userPayloadRegistration,
      }),
    }),
    confirmEmail: build.mutation<ConfirmEmailApiResponse, ConfirmEmailApiArg>({
      query: (queryArg) => ({
        url: `/api/users/confirm-email`,
        method: "POST",
        body: queryArg.userPayloadEmailConfirmation,
      }),
    }),
    login: build.mutation<LoginApiResponse, LoginApiArg>({
      query: (queryArg) => ({
        url: `/api/users/login`,
        method: "POST",
        body: queryArg.userPayloadLogin,
      }),
    }),
  }),
  overrideExisting: false,
});
export { injectedRtkApi as api };
export type CreateArticleApiResponse = /** status 201 Article successfully created */ ArticlePayloadArticle;
export type CreateArticleApiArg = {
  articlePayloadArticleCreate: ArticlePayloadArticleCreate;
};
export type ReadCurrentUserApiResponse = /** status 200 User details and fresh JWT. */ UserPayloadUser;
export type ReadCurrentUserApiArg = void;
export type UpdateCurrentUserApiResponse = /** status 200 User details and fresh JWT. */ UserPayloadUser;
export type UpdateCurrentUserApiArg = {
  userPayloadUserUpdate: UserPayloadUserUpdate;
};
export type RegisterUserApiResponse = /** status 201 User successfully created */ UserPayloadUser;
export type RegisterUserApiArg = {
  userPayloadRegistration: UserPayloadRegistration;
};
export type ConfirmEmailApiResponse = /** status 201 User's email address confirmed */ UserPayloadUser;
export type ConfirmEmailApiArg = {
  userPayloadEmailConfirmation: UserPayloadEmailConfirmation;
};
export type LoginApiResponse = /** status 200 User successfully logged in */ UserPayloadUser;
export type LoginApiArg = {
  userPayloadLogin: UserPayloadLogin;
};
export type ArticlePayloadArticle = {
  article: {
    /** Article's slug. */
    slug: string;
  };
};
export type Validation = {
  errors: {
    [key: string]: string[];
  };
};
export type ArticlePayloadArticleCreate = {
  article: {
    /** Article's contents. */
    body: string;
    /** Article's description. */
    description: string;
    /** Tags. */
    tagList: string[];
    /** Article's title.
        
        This is will be used to generate a slug for this article. */
    title: string;
  };
};
export type UserPayloadUser = {
  user: {
    /** User's biography.
        
        Empty string means biography has never been provided. */
    bio: string;
    /** User's email, e.g. `rob.pike@gmail.com`. */
    email: string;
    /** Location of user's image (if any). */
    image: string | null;
    /** Fresh JWT token. */
    token: string;
    /** User's name or nickname.
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username: string;
  };
};
export type UserPayloadUserUpdate = {
  user: {
    /** User's biography.
        
        Note that Empty string will override the existing biography. */
    bio?: string;
    /** User's email, e.g. `rob.pike@gmail.com`. */
    email?: string;
    /** New image URL.
        
        Specifying `null` means removing the image altogether. */
    image?: string | null;
    /** New password. */
    password?: string;
    /** User's name or nickname.
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username?: string;
  };
};
export type UserPayloadRegistration = {
  user: {
    /** User's email, e.g. `rob.pike@gmail.com`.
        
        This is case-insensitively unique in the system. */
    email: string;
    /** User's password.
        
        There are currently no limitations on password strength. */
    password: string;
    /** User's name or nickname.
        
        This is - just like the user's `email` - case-insensitively unique
        in the system. */
    username: string;
  };
};
export type UserPayloadEmailConfirmation = {
  user: {
    /** One-time password.
        
        An numeric code that has been sent to them upon registration. */
    otp: string;
  };
};
export type UserPayloadLogin = {
  user: {
    /** User's email, e.g. `rob.pike@gmail.com`.
        
        This is case-insensitively unique in the system. */
    email: string;
    /** User's password. */
    password: string;
  };
};
export const {
  useCreateArticleMutation,
  useReadCurrentUserQuery,
  useUpdateCurrentUserMutation,
  useRegisterUserMutation,
  useConfirmEmailMutation,
  useLoginMutation,
} = injectedRtkApi;
