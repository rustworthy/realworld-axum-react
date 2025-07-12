import { base as api } from "./base";

const injectedRtkApi = api.injectEndpoints({
  endpoints: (build) => ({
    read: build.query<ReadApiResponse, ReadApiArg>({
      query: () => ({ url: `/api/user` }),
    }),
    update: build.mutation<UpdateApiResponse, UpdateApiArg>({
      query: () => ({ url: `/api/user`, method: "PUT" }),
    }),
    handler: build.mutation<HandlerApiResponse, HandlerApiArg>({
      query: (queryArg) => ({
        url: `/api/user`,
        method: "POST",
        body: queryArg.userPayloadRegistration,
      }),
    }),
    login: build.mutation<LoginApiResponse, LoginApiArg>({
      query: () => ({ url: `/api/user/login`, method: "POST" }),
    }),
  }),
  overrideExisting: false,
});
export { injectedRtkApi as api };
export type ReadApiResponse = unknown;
export type ReadApiArg = void;
export type UpdateApiResponse = unknown;
export type UpdateApiArg = void;
export type HandlerApiResponse = /** status 201 User successfully created */ UserPayloadUser;
export type HandlerApiArg = {
  userPayloadRegistration: UserPayloadRegistration;
};
export type LoginApiResponse = unknown;
export type LoginApiArg = void;
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
        
        This is  - just like [`User::email`] - unique in the system. */
    username: string;
  };
};
export type Validation = {
  errors: {
    [key: string]: string[];
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
        
        This is  - just like [`User::email`] - unique in the system. */
    username: string;
  };
};
export const { useReadQuery, useUpdateMutation, useHandlerMutation, useLoginMutation } = injectedRtkApi;
