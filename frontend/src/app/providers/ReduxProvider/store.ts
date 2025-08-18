import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";

import { authReducer } from "@/features/auth";
import { base, loadingMiddleware } from "@/shared/api";
import { loadingReducer } from "@/shared/store/loading";

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
