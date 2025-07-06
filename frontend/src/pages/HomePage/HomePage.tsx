import { FC } from "react";

import { useListArticlesByAuthorQuery } from "../../api/base";
import type { Article } from "../../api/base";
import * as S from "./HomePage.styles";

const Feed = (props: { articles: Article[] }) => (
  <div style={{ width: "100%" }}>
    {props.articles.map((article) => (
      <Article key={article.id} article={article} />
    ))}
  </div>
);
const Article = (props: { article: Article }) => <pre>{JSON.stringify(props.article)}</pre>;

export const HomePage: FC = () => {
  const { data, isError, isLoading } = useListArticlesByAuthorQuery(1);

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.BannerTitle>conduit</S.BannerTitle>

          <S.BannerDescription>A place to share your knowledge.</S.BannerDescription>
        </S.BannerContainer>
      </S.Banner>
      {isLoading ? <span>Loading artiles</span> : isError ? <span>Error</span> : <Feed articles={data!} />}
    </S.PageWrapper>
  );
};
