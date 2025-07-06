import { FC } from "react";

import { useListArticlesByAuthorQuery } from "../../api";
import type { Article } from "../../api";
import * as S from "./HomePage.styles";

const Article = (props: { article: Article }) => <pre>{JSON.stringify(props.article)}</pre>;

export const HomePage: FC = () => {
  const { data, isLoading } = useListArticlesByAuthorQuery(1);

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.BannerTitle>conduit</S.BannerTitle>
          {isLoading ? null : data?.map((article) => <Article key={article.id} article={article} />)}
          <S.BannerDescription>A place to share your knowledge.</S.BannerDescription>
        </S.BannerContainer>
      </S.Banner>
    </S.PageWrapper>
  );
};
