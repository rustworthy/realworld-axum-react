import { FC } from "react";

import { useReadCurrentUserQuery } from "../../api/generated";
import * as S from "./HomePage.styles";

export const HomePage: FC = () => {
  useReadCurrentUserQuery();

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.BannerTitle>conduit</S.BannerTitle>
          <S.BannerDescription>A place to share your knowledge.</S.BannerDescription>
        </S.BannerContainer>
      </S.Banner>
    </S.PageWrapper>
  );
};
