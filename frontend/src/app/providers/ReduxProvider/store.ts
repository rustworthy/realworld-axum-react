import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";

import { authReducer } from "@/features/auth";
import { loadingReducer } from "@/features/loading";
import { base } from "@/shared/api";

import { loadingMiddleware } from "./middlewares/loading.middleware";

export const store = configureStore({
  reducer: {
    [base.reducerPath]: base.reducer,
    auth: authReducer,
    loading: loadingReducer,
  },
  // this way we are enabling caching, invalidation, polling,
  // and other feature of `rtk-query`
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(base.middleware, loadingMiddleware),
});

// required for `refetchOnFocus/refetchOnReconnect` functionality, see:
// https://redux-toolkit.js.org/rtk-query/api/setupListeners
setupListeners(store.dispatch);
