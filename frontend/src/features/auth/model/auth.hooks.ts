import { useCallback, useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useNavigate } from "react-router";

import { SerializedError } from "@reduxjs/toolkit";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useConfirmEmailMutation, useLoginMutation, useUpdateCurrentUserMutation } from "@/shared/api";
import type { ConfirmEmailApiArg, LoginApiArg, UpdateCurrentUserApiArg, UserPayloadUser } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { AppState } from "@/shared/types/store.types";
import z, { ZodError } from "zod";

import { authSliceLoggedOutState, restoreSnapshot, setLoggedIn, setLoggedOut } from "./authSlice";
import { type AuthSliceState, authSliceSchema } from "./authSlice.schema";

export type MaybeUserPayload =
  | { data: UserPayloadUser; error: undefined }
  | { data: undefined; error: FetchBaseQueryError | SerializedError };
export type ConfirmEmailFnType = (arg: ConfirmEmailApiArg) => Promise<MaybeUserPayload>;
export type LoginFnType = (arg: LoginApiArg) => Promise<MaybeUserPayload>;
export type UpdateCurrentUserFnType = (arg: UpdateCurrentUserApiArg) => Promise<MaybeUserPayload>;

export type UseAuthSnapshotRestorationReturnType = {
  isRestoring: boolean;
};

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

export const useAuthSnapshotRestoration = (): UseAuthSnapshotRestorationReturnType => {
  const [isRestoring, setIsRestoring] = useState<boolean>(true);
  const navigate = useNavigate();
  const dispatch = useDispatch();

  useEffect(() => {
    try {
      const value = localStorage.getItem("user");
      if (value === null) return;
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
    } finally {
      setIsRestoring(false);
    }
  }, []);

  return { isRestoring };
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
