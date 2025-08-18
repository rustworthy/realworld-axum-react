import { Middleware } from "@reduxjs/toolkit";

import { startLoading, stopLoading } from "@/shared/store/loading";
import { ANY_TODO } from "@/shared/types/common.types";

interface LoadingMiddlewareAction {
  type: string;
  meta?: Record<string, ANY_TODO>;
}

export const loadingMiddleware: Middleware = (storeAPI) => (next) => (action) => {
  const actionAny = action as LoadingMiddlewareAction;
  const actionType = actionAny.type;

  if (actionType.endsWith("/pending")) {
    const timer = setTimeout(() => {
      storeAPI.dispatch(startLoading());
    }, 200);

    actionAny.meta = { ...actionAny.meta, loaderTimer: timer };
  }

  if (actionType.endsWith("/fulfilled") || actionType.endsWith("/rejected")) {
    const timer = actionAny.meta?.loaderTimer;
    if (timer) clearTimeout(timer);

    storeAPI.dispatch(stopLoading());
  }

  return next(action);
};
