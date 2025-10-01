import { FC } from "react";
import { Link } from "react-router";

import { ArticlePayloadArticle } from "@/shared/api";
import { formatDate, truncateText } from "@/shared/lib/utils";

import { Avatar } from "../Avatar";
import * as S from "./AuthorInfo.styles";

export type AuthorInfoProps = {
  article: ArticlePayloadArticle["article"];
};

export const AuthorInfo: FC<AuthorInfoProps> = ({ article }) => {
  const username = article.author.username;
  const profilePath = `/profile/${username}`;

  return (
    <S.AuthorInfo>
      <Link to={profilePath}>
        <Avatar imageUrl={article.author.image} username={username} />
      </Link>
      <S.AuthorInfoNameBlock>
        <S.AuthorName to={profilePath}>{truncateText(username, 20)}</S.AuthorName>
        <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
      </S.AuthorInfoNameBlock>
    </S.AuthorInfo>
  );
};
