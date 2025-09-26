import { FC } from "react";

import { ArticlePayloadArticle } from "@/shared/api";
import { formatDate, truncateText } from "@/shared/lib/utils";

import * as S from "./AuthorInfo.styles";
import { Link } from "react-router";

export type AuthorInfoProps = {
  article: ArticlePayloadArticle["article"];
};

export const AuthorInfo: FC<AuthorInfoProps> = ({ article }) => {
  const username = article.author.username;
  const profilePath = `/profile/${username}`;

  return (
    <S.AuthorInfo>
      <Link to={profilePath}>
        <S.AuthorImage
          src={article.author.image ?? "https://avatars.githubusercontent.com/u/4324516?v=4"}
          alt={`${username}'s profile picture`}
        />
      </Link>
      <S.AuthorInfoNameBlock>
        <S.AuthorName to={profilePath}>{truncateText(username, 20)}</S.AuthorName>
        <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
      </S.AuthorInfoNameBlock>
    </S.AuthorInfo>
  );
};
