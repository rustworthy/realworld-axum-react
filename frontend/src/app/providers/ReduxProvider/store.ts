import { configureStore } from "@reduxjs/toolkit";
import { setupListeners } from "@reduxjs/toolkit/query";

import { base } from "@/shared/api";

export const store = configureStore({
  reducer: {
    [base.reducerPath]: base.reducer,
  },

  // this way we are enabling caching, invalidation, polling,
  // and other feature of `rtk-query`
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(base.middleware),
});

// required for `refetchOnFocus/refetchOnReconnect` functionality, see:
// https://redux-toolkit.js.org/rtk-query/api/setupListeners
setupListeners(store.dispatch);
