import { FC, PropsWithChildren } from "react";

import * as S from "./AuthPageLayout.styles";

/**
 * Wrapper for authentication related pages: signin, signup,
 * password reset, email confirmation, etc.
 */
export const AuthPageLayout: FC<PropsWithChildren<{ title: string }>> = (props) => {
  return (
    <S.PageWrapper>
      <S.Title>{props.title}</S.Title>
      <S.PageInner>{props.children}</S.PageInner>
    </S.PageWrapper>
  );
};
