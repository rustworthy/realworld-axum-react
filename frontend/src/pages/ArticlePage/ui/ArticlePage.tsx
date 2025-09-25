import { FC } from "react";
import { useParams } from "react-router";

import { useAuth } from "@/features/auth";
import { NotFoundPage } from "@/pages/NotFoundPage";
import { useReadArticleQuery } from "@/shared/api";
import { TagList } from "@/shared/ui/TagList";
import MDEditor from "@uiw/react-md-editor";
import { useTernaryDarkMode } from "usehooks-ts";

import { ArticleComments } from "./ArticleComments";
import { ArticleMeta } from "./ArticleMeta";
import * as S from "./ArticlePage.styles";

const IS_COMMENT_FEAUTURE_FINISHED = false;

export const ArticlePage: FC = () => {
  const { slug } = useParams<{ slug: string }>();
  if (!slug) return <NotFoundPage />;
  const { data, isLoading } = useReadArticleQuery({ slug: slug! });
  const { user } = useAuth();
  const { isDarkMode } = useTernaryDarkMode();

  if (!data) return isLoading ? null : <NotFoundPage />;
  const article = data.article;

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.ArticleTitle>{article.title}</S.ArticleTitle>
          <ArticleMeta article={article} user={user} />
        </S.BannerContainer>
      </S.Banner>

      <S.MainContent>
        <S.ArticleContent data-color-mode={isDarkMode ? "dark" : "light"}>
          <MDEditor.Markdown source={article.body} />
          <TagList tags={article.tagList} />
        </S.ArticleContent>

        <S.Separator />

        <ArticleMeta article={article} user={user} />
        {IS_COMMENT_FEAUTURE_FINISHED ? <ArticleComments user={user} /> : null}
      </S.MainContent>
    </S.PageWrapper>
  );
};
