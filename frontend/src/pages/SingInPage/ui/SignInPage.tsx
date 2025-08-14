import { Navigate } from "react-router";

import { useAuth } from "@/shared/auth";
import { ROUTES } from "@/shared/constants/routes.constants";

import * as S from "./SignInPage.styles";

export const SignInPage = () => {
  const { isAuthenticated } = useAuth();

  if (isAuthenticated) return <Navigate to={ROUTES.HOME} />;

  return <S.Title> SignInPage</S.Title>;
};
