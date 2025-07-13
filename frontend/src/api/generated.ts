import { base as api } from "./base";

const injectedRtkApi = api.injectEndpoints({
  endpoints: (build) => ({
    readCurrentUser: build.query<ReadCurrentUserApiResponse, ReadCurrentUserApiArg>({
      query: () => ({ url: `/api/user/` }),
    }),
    updateCurrentUser: build.mutation<UpdateCurrentUserApiResponse, UpdateCurrentUserApiArg>({
      query: () => ({ url: `/api/user/`, method: "PUT" }),
    }),
    createUser: build.mutation<CreateUserApiResponse, CreateUserApiArg>({
      query: () => ({ url: `/api/user/`, method: "POST" }),
    }),
    login: build.mutation<LoginApiResponse, LoginApiArg>({
      query: () => ({ url: `/api/user/login`, method: "POST" }),
    }),
  }),
  overrideExisting: false,
});
export { injectedRtkApi as api };
export type ReadCurrentUserApiResponse = unknown;
export type ReadCurrentUserApiArg = void;
export type UpdateCurrentUserApiResponse = unknown;
export type UpdateCurrentUserApiArg = void;
export type CreateUserApiResponse = unknown;
export type CreateUserApiArg = void;
export type LoginApiResponse = unknown;
export type LoginApiArg = void;
export const { useReadCurrentUserQuery, useUpdateCurrentUserMutation, useCreateUserMutation, useLoginMutation } = injectedRtkApi;
