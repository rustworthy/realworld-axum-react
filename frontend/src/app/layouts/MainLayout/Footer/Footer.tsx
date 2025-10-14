import { FC } from "react";

import { ROUTES } from "@/shared/constants/routes.constants";

import * as S from "./Footer.styles";

export const Footer: FC = () => {
  return (
    <S.Footer>
      <S.FooterContainer>
        <S.FooterContent>
          <S.LogoLink to={ROUTES.HOME}>conduit</S.LogoLink> An interactive learning project from{" "}
          <a href="https://thinkster.io" target="_blank">
            Thinkster
          </a>
          . Code & design licensed under MIT.
        </S.FooterContent>
      </S.FooterContainer>
    </S.Footer>
  );
};
