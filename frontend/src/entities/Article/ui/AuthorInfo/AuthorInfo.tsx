import { FC } from "react";
import { Link } from "react-router";

import { formatDate, truncateText } from "@/shared/lib/utils";
import { Avatar } from "@/shared/ui/Avatar";

import * as S from "./AuthorInfo.styles";

export interface IAuthorInfoProps {
  username: string;
  imageUrl?: string | null;
  authoredAt?: string;
}

export const AuthorInfo: FC<IAuthorInfoProps> = ({ imageUrl, username, authoredAt }) => {
  const profilePath = `/profile/${username}`;

  return (
    <S.AuthorInfo>
      <Link to={profilePath}>
        <Avatar imageUrl={imageUrl} username={username} />
      </Link>
      <S.AuthorInfoNameBlock>
        <S.AuthorName to={profilePath}>{truncateText(username, 20)}</S.AuthorName>
        {authoredAt ? <S.ArticleDate>{formatDate(authoredAt)}</S.ArticleDate> : null}
      </S.AuthorInfoNameBlock>
    </S.AuthorInfo>
  );
};
