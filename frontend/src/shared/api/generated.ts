import { base as api } from "./base";

const injectedRtkApi = api.injectEndpoints({
  endpoints: (build) => ({
    listArticles: build.query<ListArticlesApiResponse, ListArticlesApiArg>({
      query: (queryArg) => ({
        url: `/api/articles`,
        params: {
          tag: queryArg.tag,
          author: queryArg.author,
          favorited: queryArg.favorited,
          limit: queryArg.limit,
          offset: queryArg.offset,
        },
      }),
    }),
    createArticle: build.mutation<CreateArticleApiResponse, CreateArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles`,
        method: "POST",
        body: queryArg.articlePayloadArticleCreate,
      }),
    }),
    personalFeed: build.query<PersonalFeedApiResponse, PersonalFeedApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/feed`,
        params: {
          tag: queryArg.tag,
          author: queryArg.author,
          favorited: queryArg.favorited,
          limit: queryArg.limit,
          offset: queryArg.offset,
        },
      }),
    }),
    readArticle: build.query<ReadArticleApiResponse, ReadArticleApiArg>({
      query: (queryArg) => ({ url: `/api/articles/${queryArg.slug}` }),
    }),
    updateArticle: build.mutation<UpdateArticleApiResponse, UpdateArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}`,
        method: "PUT",
        body: queryArg.articlePayloadArticleUpdate,
      }),
    }),
    deleteArticle: build.mutation<DeleteArticleApiResponse, DeleteArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}`,
        method: "DELETE",
      }),
    }),
    listComments: build.query<ListCommentsApiResponse, ListCommentsApiArg>({
      query: (queryArg) => ({ url: `/api/articles/${queryArg.slug}/comments` }),
    }),
    createComment: build.mutation<CreateCommentApiResponse, CreateCommentApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}/comments`,
        method: "POST",
        body: queryArg.commentPayloadCommentCreate,
      }),
    }),
    deleteComment: build.mutation<DeleteCommentApiResponse, DeleteCommentApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}/comments/${queryArg.commentId}`,
        method: "DELETE",
      }),
    }),
    favoriteArticle: build.mutation<FavoriteArticleApiResponse, FavoriteArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}/favorite`,
        method: "POST",
      }),
    }),
    unfavoriteArticle: build.mutation<UnfavoriteArticleApiResponse, UnfavoriteArticleApiArg>({
      query: (queryArg) => ({
        url: `/api/articles/${queryArg.slug}/favorite`,
        method: "DELETE",
      }),
    }),
    profile: build.query<ProfileApiResponse, ProfileApiArg>({
      query: (queryArg) => ({ url: `/api/profiles/${queryArg.username}` }),
    }),
    followProfile: build.mutation<FollowProfileApiResponse, FollowProfileApiArg>({
      query: (queryArg) => ({
        url: `/api/profiles/${queryArg.username}/follow`,
        method: "POST",
      }),
    }),
    unfollowProfile: build.mutation<UnfollowProfileApiResponse, UnfollowProfileApiArg>({
      query: (queryArg) => ({
        url: `/api/profiles/${queryArg.username}/follow`,
        method: "DELETE",
      }),
    }),
    listTags: build.query<ListTagsApiResponse, ListTagsApiArg>({
      query: () => ({ url: `/api/tags` }),
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
export type ListArticlesApiResponse = /** status 200 Articles list successfully retrieved */ ArticlesList;
export type ListArticlesApiArg = {
  /** Filter articles by tag. */
  tag?: string;
  /** Filter articles by author (username). */
  author?: string;
  /** Filter articles favorited by user (username). */
  favorited?: string;
  /** Limit number of returned articles. */
  limit?: number;
  /** Offset/skip number of articles. */
  offset?: number;
};
export type CreateArticleApiResponse = /** status 201 Article successfully created */ ArticlePayloadArticle;
export type CreateArticleApiArg = {
  articlePayloadArticleCreate: ArticlePayloadArticleCreate;
};
export type PersonalFeedApiResponse = /** status 200 Articles list successfully retrieved */ ArticlesList;
export type PersonalFeedApiArg = {
  /** Filter articles by tag. */
  tag?: string;
  /** Filter articles by author (username). */
  author?: string;
  /** Filter articles favorited by user (username). */
  favorited?: string;
  /** Limit number of returned articles. */
  limit?: number;
  /** Offset/skip number of articles. */
  offset?: number;
};
export type ReadArticleApiResponse = /** status 200 Article successfully retrieved */ ArticlePayloadArticle;
export type ReadArticleApiArg = {
  /** Article slug identifier */
  slug: string;
};
export type UpdateArticleApiResponse = /** status 200 Article successfully updated */ ArticlePayloadArticle;
export type UpdateArticleApiArg = {
  /** Article's slug identifier. */
  slug: string;
  articlePayloadArticleUpdate: ArticlePayloadArticleUpdate;
};
export type DeleteArticleApiResponse = unknown;
export type DeleteArticleApiArg = {
  /** Article's slug identifier. */
  slug: string;
};
export type ListCommentsApiResponse = /** status 200 Comments list successfully retrieved */ CommentsList;
export type ListCommentsApiArg = {
  /** Article's slug identifier. */
  slug: string;
};
export type CreateCommentApiResponse = /** status 200 Comment successfully created */ CommentPayloadComment;
export type CreateCommentApiArg = {
  /** Article's slug identifier. */
  slug: string;
  commentPayloadCommentCreate: CommentPayloadCommentCreate;
};
export type DeleteCommentApiResponse = unknown;
export type DeleteCommentApiArg = {
  /** Article's slug identifier. */
  slug: string;
  commentId: string;
};
export type FavoriteArticleApiResponse = /** status 200 Article successfully updated */ ArticlePayloadArticle;
export type FavoriteArticleApiArg = {
  /** Article's slug identifier. */
  slug: string;
};
export type UnfavoriteArticleApiResponse = /** status 200 Article successfully updated */ ArticlePayloadArticle;
export type UnfavoriteArticleApiArg = {
  /** Article's slug identifier. */
  slug: string;
};
export type ProfileApiResponse = /** status 200 User profile successfully retrieved */ UserProfilePayloadUserProfile;
export type ProfileApiArg = {
  username: string;
};
export type FollowProfileApiResponse =
  /** status 200 User successfully started follow current user's profile */ UserProfilePayloadUserProfile;
export type FollowProfileApiArg = {
  username: string;
};
export type UnfollowProfileApiResponse =
  /** status 200 User successfully unfollow from current user's profile */ UserProfilePayloadUserProfile;
export type UnfollowProfileApiArg = {
  username: string;
};
export type ListTagsApiResponse = /** status 200 Tags list successfully retrieved */ TagsList;
export type ListTagsApiArg = void;
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
export type Author = {
  /** User's biography.
    
    Empty string means biography has never been provided. */
  bio: string;
  /** If the current user is following the author. */
  following: boolean;
  /** Location of user's image (if any). */
  image: string | null;
  /** User's name or nickname.
    
    This is  - just like the user's `email` - case-insensitively unique
    in the system. */
  username: string;
};
export type Article = {
  /** The article's author details. */
  author: Author;
  /** Article's contents. */
  body: string;
  /** When this article was created. */
  createdAt: string;
  /** Article's description. */
  description: string;
  /** If this article is favorited by the current user. */
  favorited: boolean;
  /** How many users favorited this article. */
  favoritesCount: number;
  /** Article's slug. */
  slug: string;
  /** Tags. */
  tagList: string[];
  /** Article's title.
    
    This is will be used to generate a slug for this article. */
  title: string;
  /** When this article was last update. */
  updatedAt: string;
};
export type ArticlesList = {
  /** List of articles. */
  articles: Article[];
  articlesCount: number;
};
export type ArticlePayloadArticle = {
  article: {
    /** The article's author details. */
    author: Author;
    /** Article's contents. */
    body: string;
    /** When this article was created. */
    createdAt: string;
    /** Article's description. */
    description: string;
    /** If this article is favorited by the current user. */
    favorited: boolean;
    /** How many users favorited this article. */
    favoritesCount: number;
    /** Article's slug. */
    slug: string;
    /** Tags. */
    tagList: string[];
    /** Article's title.
        
        This is will be used to generate a slug for this article. */
    title: string;
    /** When this article was last update. */
    updatedAt: string;
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
export type ArticlePayloadArticleUpdate = {
  article: {
    /** Article's contents. */
    body?: string;
    /** Article's description. */
    description?: string;
    /** Tags. */
    tagList?: string[];
    /** Article's title.
        
        This is will be used to generate a slug for this article. */
    title?: string;
  };
};
export type Comment = {
  /** Details of the comment's author */
  author: Author;
  /** Comment's text. */
  body: string;
  /** When this comment was created. */
  createdAt: string;
  /** Comment's unique identifier. */
  id: string;
  /** When this comment was last update. */
  updatedAt: string;
};
export type CommentsList = {
  /** List of comments. */
  comments: Comment[];
};
export type CommentPayloadComment = {
  comment: {
    /** Details of the comment's author */
    author: Author;
    /** Comment's text. */
    body: string;
    /** When this comment was created. */
    createdAt: string;
    /** Comment's unique identifier. */
    id: string;
    /** When this comment was last update. */
    updatedAt: string;
  };
};
export type CommentPayloadCommentCreate = {
  comment: {
    /** Comment's text. */
    body: string;
  };
};
export type UserProfilePayloadUserProfile = {
  profile: {
    /** User's biography.
        
        Empty string means biography has never been provided. */
    bio: string;
    /** Following, if the current user is subscribed to the searched user */
    following: boolean;
    /** Location of user's image (if any). */
    image: string | null;
    /** User's name or nickname.
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username: string;
  };
};
export type TagsList = {
  /** List of tags. */
  tags: string[];
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
    /** Turnstile captcha token. */
    captcha: string;
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
    /** Turnstile captcha token. */
    captcha: string;
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
