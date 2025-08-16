import { useCallback, useEffect } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router";

import { SerializedError } from "@reduxjs/toolkit";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import z, { ZodError } from "zod";

import {
  ConfirmEmailApiArg,
  LoginApiArg,
  UpdateCurrentUserApiArg,
  UserPayloadUser,
  useConfirmEmailMutation,
  useLoginMutation,
  useUpdateCurrentUserMutation,
} from "../api/generated";
import { ROUTES } from "../constants/routes.constants";
import { AppState } from "../providers/ReduxProvider/store";
import {
  type AuthSliceState,
  authSliceLoggedOutState,
  authSliceSchema,
  restoreSnapshot,
  setLoggedIn,
  setLoggedOut,
} from "./slice";

export type MaybeUserPayload =
  | { data: UserPayloadUser; error: undefined }
  | { data: undefined; error: FetchBaseQueryError | SerializedError };
export type ConfirmEmailFnType = (arg: ConfirmEmailApiArg) => Promise<MaybeUserPayload>;
export type LoginFnType = (arg: LoginApiArg) => Promise<MaybeUserPayload>;
export type UpdateCurrentUserFnType = (arg: UpdateCurrentUserApiArg) => Promise<MaybeUserPayload>;

export type UseAuthHookReturnType = {
  user: AuthSliceState["user"];
  isAuthenticated: boolean;
  confirmEmail: ConfirmEmailFnType;
  isConfirmEmailLoading: boolean;
  login: LoginFnType;
  isLoginLoading: boolean;
  update: UpdateCurrentUserFnType;
  isUpdateLoading: boolean;
  logout: () => void;
};

const AUTH_SNAPSHOT_KEY = "user";

export const useAuthSnapshotRestoration = () => {
  const navigate = useNavigate();
  const dispatch = useDispatch();

  useEffect(() => {
    const value = localStorage.getItem("user");
    if (value === null) return;
    try {
      const auth = authSliceSchema.parse(JSON.parse(value));
      dispatch(restoreSnapshot(auth));
    } catch (error) {
      if (error instanceof ZodError) {
        /* eslint-disable no-console */
        console.warn(z.treeifyError(error));
      } else {
        console.warn(error);
        /* eslint-enable no-console */
      }
      localStorage.removeItem("user");
      navigate(ROUTES.SIGNIN);
    }
  }, []);
};

export const useAuth = (): UseAuthHookReturnType => {
  const auth: AuthSliceState = useSelector((state: AppState) => state.auth);
  const dispatch = useDispatch();
  const [updateMutation, { isLoading: isUpdateLoading }] = useUpdateCurrentUserMutation();
  const [confirmEmailMutation, { isLoading: isConfirmEmailLoading }] = useConfirmEmailMutation();
  const [loginMutation, { isLoading: isLoginLoading }] = useLoginMutation();

  const makeMutateFn = useCallback(
    <A>(mutate: (arg: A) => Promise<MaybeUserPayload>) =>
      useCallback(async (arg: A) => {
        const result = await mutate(arg);
        if (result.data) {
          localStorage.setItem(AUTH_SNAPSHOT_KEY, JSON.stringify({ isAuthenticated: true, user: result.data.user }));
          dispatch(setLoggedIn(result.data));
        }
        return result;
      }, []),
    [],
  );

  const confirmEmail = makeMutateFn(confirmEmailMutation);
  const update = makeMutateFn(updateMutation);
  const login = makeMutateFn(loginMutation);
  const logout = useCallback(() => {
    localStorage.setItem(AUTH_SNAPSHOT_KEY, JSON.stringify(authSliceLoggedOutState));
    dispatch(setLoggedOut());
  }, []);

  return {
    user: auth.user,
    isAuthenticated: auth.isAuthenticated,

    confirmEmail,
    isConfirmEmailLoading,

    update,
    isUpdateLoading,

    login,
    isLoginLoading,
    logout,
  };
};
