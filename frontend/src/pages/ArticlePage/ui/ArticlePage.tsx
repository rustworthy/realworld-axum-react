import { FC } from "react";
import { useLocation, useParams } from "react-router";

import { useAuth } from "@/features/auth";
import { NotFoundPage } from "@/pages/NotFoundPage";
import { useListCommentsQuery, useReadArticleQuery } from "@/shared/api";
import { useHashScrollIn } from "@/shared/lib/hooks/navigation";
import { AuthorInfo, TagList } from "@/shared/ui/Article";
import MDEditor from "@uiw/react-md-editor";
import { useTernaryDarkMode } from "usehooks-ts";

import { ArticleMeta } from "./ArticleMeta";
import * as S from "./ArticlePage.styles";
import { CommentForm } from "./CommentForm";

/**
 * Adjust links to work with browser router.
 *
 * @see https://github.com/uiwjs/react-md-editor/issues/356
 */
function urlTransform(link: string, path: string): string {
  return link.startsWith("#") ? `${path}${link}` : link;
}

export const ArticlePage: FC = () => {
  const { slug } = useParams<{ slug: string }>();
  if (!slug) return <NotFoundPage />;
  const location = useLocation();
  const { data, isLoading } = useReadArticleQuery({ slug: slug! });
  const { data: commentsData, isLoading: isCommentsDataLoading } = useListCommentsQuery({ slug: slug! });
  const { user } = useAuth();
  const { isDarkMode } = useTernaryDarkMode();
  useHashScrollIn(data);

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
          <MDEditor.Markdown source={article.body} urlTransform={(url) => urlTransform(url, location.pathname)} />
          <TagList tags={article.tagList} />
        </S.ArticleContent>
        <S.Separator />
        <ArticleMeta article={article} user={user} />
      </S.MainContent>

      <S.CommentsContainer>
        {user ? <CommentForm slug={slug!} user={user} /> : null}
        <div>
          {isCommentsDataLoading ? (
            <p>Loading comments...</p>
          ) : (
            commentsData?.comments.map((comment) => (
              <S.Comment key={comment.id}>
                <S.CommentBody>{comment.body}</S.CommentBody>
                <S.CommentFooter>
                  <AuthorInfo imageUrl={comment.author.image} username={comment.author.username} authoredAt={comment.createdAt} />
                </S.CommentFooter>
              </S.Comment>
            ))
          )}
        </div>
      </S.CommentsContainer>
    </S.PageWrapper>
  );
};
