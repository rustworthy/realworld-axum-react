import { FC } from "react";
import { useParams } from "react-router";

import { useAuth } from "@/features/auth";
import { NotFoundPage } from "@/pages/NotFoundPage";
import { ArticlePayloadArticle, UserPayloadUser, useReadArticleQuery } from "@/shared/api/generated";
import MDEditor from "@uiw/react-md-editor";
import { useTernaryDarkMode } from "usehooks-ts";

import * as S from "./ArticlePage.styles";

const IS_COMMENT_FEAUTURE_FINISHED = false;

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

const ArticleMeta: FC<{ article: ArticlePayloadArticle["article"]; isAuthor: boolean }> = ({ article, isAuthor }) => {
  const username = article.author.username;
  const profilePath = `/profile/${username}`;
  return (
    <S.ArticleMeta>
      <S.AuthorInfo>
        <a href={profilePath}>
          <S.AuthorImage
            src={article.author.image ?? "https://avatars.githubusercontent.com/u/4324516?v=4"}
            alt={`${username}'s profile picture`}
          />
        </a>
        <S.AuthorInfoNameBlock>
          <S.AuthorName href={profilePath}>{username}</S.AuthorName>
          <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
        </S.AuthorInfoNameBlock>
      </S.AuthorInfo>
      <S.ArticleActions>
        <S.ActionButton disabled={isAuthor} className="btn-outline-secondary">
          <i className="ion-plus-round" />
          Follow {article.author.username}
        </S.ActionButton>
        <S.ActionButton disabled={isAuthor} className="btn-outline-primary">
          <i className="ion-heart" />
          Favorite Article <span>({article.favoritesCount})</span>
        </S.ActionButton>
        <S.ActionButton disabled={!isAuthor} className="btn-outline-secondary">
          <i className="ion-edit" />
          Edit Article
        </S.ActionButton>
        <S.ActionButton disabled={!isAuthor} className="btn-outline-danger">
          <i className="ion-trash-a" />
          Delete Article
        </S.ActionButton>
      </S.ArticleActions>
    </S.ArticleMeta>
  );
};

const ArticleComments: FC<{ user: UserPayloadUser["user"] | null }> = ({ user }) => {
  const comments = [
    {
      id: 1,
      createdAt: "2016-02-18T03:22:56.637Z",
      updatedAt: "2016-02-18T03:22:56.637Z",
      body: "With supporting text below as a natural lead-in to additional content.",
      author: {
        username: "jacob-schmidt",
        bio: "I work at statefarm",
        image: "http://i.imgur.com/Qr71crq.jpg",
        following: false,
      },
    },
  ];
  return (
    <S.CommentSection>
      {user && (
        <S.CommentForm>
          <S.CommentFormBody>
            <S.CommentTextarea placeholder="Write a comment..." rows={3} />
          </S.CommentFormBody>
          <S.CommentFormFooter>
            <S.CommentAuthorImage src={user.image || ""} alt={user.username} />
            <S.CommentButton type="submit">Post Comment</S.CommentButton>
          </S.CommentFormFooter>
        </S.CommentForm>
      )}

      {comments.map((comment) => (
        <S.Comment key={comment.id}>
          <S.CommentBody>{comment.body}</S.CommentBody>
          <S.CommentFooter>
            <S.CommentAuthorImage src={comment.author.image} alt={comment.author.username} />
            <S.CommentAuthor href={`/profile/${comment.author.username}`}>{comment.author.username}</S.CommentAuthor>
            <S.CommentDate>{formatDate(comment.createdAt)}</S.CommentDate>
            {user?.username === comment.author.username && (
              <S.ModOptions>
                <i className="ion-trash-a" />
              </S.ModOptions>
            )}
          </S.CommentFooter>
        </S.Comment>
      ))}
    </S.CommentSection>
  );
};

export const ArticlePage: FC = () => {
  const { slug } = useParams<{ slug: string }>();
  if (!slug) return <NotFoundPage />;
  const { data } = useReadArticleQuery({ slug: slug! });
  const { user } = useAuth();
  const { isDarkMode } = useTernaryDarkMode();

  if (!data) return null;
  const article = data.article;
  const isAuthor = user?.username === article.author.username;

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.ArticleTitle>{article.title}</S.ArticleTitle>
          <ArticleMeta article={article} isAuthor={isAuthor} />
        </S.BannerContainer>
      </S.Banner>

      <S.MainContent>
        <S.ArticleContent data-color-mode={isDarkMode ? "dark" : "light"}>
          <MDEditor.Markdown source={article.body} />
          <S.TagList>
            {article.tagList.map((tag) => (
              <S.Tag key={tag}>{tag}</S.Tag>
            ))}
          </S.TagList>
        </S.ArticleContent>

        <S.Separator />

        <ArticleMeta article={article} isAuthor={isAuthor} />
        {IS_COMMENT_FEAUTURE_FINISHED ? <ArticleComments user={user} /> : null}
      </S.MainContent>
    </S.PageWrapper>
  );
};
