import { Middleware } from "@reduxjs/toolkit";

import { startLoading, stopLoading } from "@/shared/store/loading";

type LoadingAction = ReturnType<typeof startLoading> | ReturnType<typeof stopLoading>;

export const loadingMiddleware: Middleware = (storeAPI) => (next) => (action) => {
  const actionType = (action as LoadingAction).type;

  if (actionType.endsWith("/pending")) {
    storeAPI.dispatch(startLoading());
  }
  if (actionType.endsWith("/fulfilled") || actionType.endsWith("/rejected")) {
    storeAPI.dispatch(stopLoading());
  }

  return next(action);
};
