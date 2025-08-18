import { useDispatch, useSelector } from "react-redux";

import { AppDispatch, AppState } from "@/shared/types/store.types";

export const useAppDispatch = useDispatch.withTypes<AppDispatch>();
export const useAppSelector = useSelector.withTypes<AppState>();
