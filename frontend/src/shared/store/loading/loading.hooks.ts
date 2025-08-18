import { useAppSelector } from "@/shared/lib/hooks/redux";

import { isLoading } from "./loadingSlice";

export const useIsGlobalLoading = (): boolean => useAppSelector(isLoading);
