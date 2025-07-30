import { base as api } from "./base";

const injectedRtkApi = api.injectEndpoints({
  endpoints: (build) => ({
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
export type LoginApiResponse = /** status 200 User successfully logged in */ UserPayloadUser;
export type LoginApiArg = {
  userPayloadLogin: UserPayloadLogin;
};
export type UserPayloadUser = {
  user: {
    /** User's biography.
        
        Empty string means biography has never been provided. */
    bio: string;
    /** User's email, e.g. `rob.pike@gmail.com`. */
    email: string;
    /** Location of user's image (if any). */
    image?: string | null;
    /** Fresh JWT token. */
    token: string;
    /** User's name or nickname.
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username: string;
  };
};
export type Validation = {
  errors: {
    [key: string]: string[];
  };
};
export type UserPayloadUserUpdate = {
  user: {
    /** User's biography.
        
        Empty string means biography has never been provided. */
    bio?: string | null;
    /** User's email, e.g. `rob.pike@gmail.com`. */
    email?: string | null;
    /** User's name or nickname.
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username?: string | null;
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
        
        This is  - just like the user's `email` - case-insensitively unique
        in the system. */
    username: string;
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
export const { useReadCurrentUserQuery, useUpdateCurrentUserMutation, useRegisterUserMutation, useLoginMutation } =
  injectedRtkApi;
