import { Middleware } from "@reduxjs/toolkit";

import { startLoading, stopLoading } from "@/features/loading";
import { ANY_TODO } from "@/shared/types/common.types";

interface LoadingMiddlewareAction {
  type: string;
  payload?: ANY_TODO;
  meta: {
    requestId: string;
    requestStatus: "pending" | "fulfilled" | "rejected";
    startedTimeStamp: number;
    RTK_autoBatch: boolean;
    arg: {
      endpointName: string;
      fixedCacheKey?: string;
      track: boolean;
      type: "query" | "mutation";
      originalArgs: Record<string, ANY_TODO>;
    };
  };
}

const timers: Record<string, NodeJS.Timeout> = {};

export const loadingMiddleware: Middleware = (storeAPI) => (next) => (action) => {
  const actionAny = action as LoadingMiddlewareAction;
  const actionType = actionAny.type;

  if (actionType.endsWith("/pending")) {
    if (actionAny.meta.requestId) {
      timers[actionAny.meta.requestId] = setTimeout(() => {
        storeAPI.dispatch(startLoading());
      }, 500);
    } else {
      storeAPI.dispatch(startLoading());
    }
  }

  if (actionType.endsWith("/fulfilled") || actionType.endsWith("/rejected")) {
    const timer = actionAny.meta.requestId ? timers[actionAny.meta.requestId] : null;
    if (timer) {
      clearTimeout(timer);
      delete timers[actionAny.meta.requestId];
    }

    storeAPI.dispatch(stopLoading());
  }

  return next(action);
};
