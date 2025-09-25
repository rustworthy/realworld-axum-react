import { FC, useCallback } from "react";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useDeleteArticleMutation } from "@/shared/api";
import type { ArticlePayloadArticle, UserPayloadUser } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { ANY_TODO } from "@/shared/types/common.types";
import { HeartIcon, Pencil2Icon, PlusCircledIcon, TrashIcon } from "@radix-ui/react-icons";
import { toast } from "sonner";

import * as S from "./ArticlePage.styles";
import { formatDate } from "./utils";

export type ArticleMetaProps = {
  article: ArticlePayloadArticle["article"];
  user: UserPayloadUser["user"] | null;
};

/**
 * Article's metadata and actions panel.
 */
export const ArticleMeta: FC<ArticleMetaProps> = ({ article, user }) => {
  const navigate = useNavigate();
  const [deleteArticle, { isLoading: isDeleteLoading }] = useDeleteArticleMutation();

  const username = article.author.username;
  const profilePath = `/profile/${username}`;
  const isLoading = isDeleteLoading;
  const isAuthor = username === user?.username;

  const performAction = useCallback(
    async (action: string) => {
      switch (action) {
        case "delete": {
          const result = await deleteArticle({ slug: article.slug });
          if (result.error) {
            if ((result.error as FetchBaseQueryError).status === 422) {
              // TODO: think about how to simplify extracting error messages
              const fieldType = Object.keys((result.error as ANY_TODO).data?.errors)[0];
              toast.error(`Action failed. Reason: ${(result.error as ANY_TODO).data?.errors?.[fieldType]?.[0]}`);
            }
            if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
              toast.error("Action failed. Please check your internet connection and retry.");
            }
            return;
          }
          toast.success("Your article has been delete.");
          navigate(ROUTES.EDITOR);
          break;
        }
        case "edit":
          navigate(`${ROUTES.EDITOR}/${article.slug}`);
          return;
        default:
          return;
      }
    },
    [article],
  );

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
        <S.ActionButton disabled={isAuthor || isLoading} className="btn-outline-secondary">
          <PlusCircledIcon />
          Follow {article.author.username}
        </S.ActionButton>

        <S.ActionButton disabled={isAuthor || isLoading} className="btn-outline-primary">
          <HeartIcon />
          Favorite Article <span>({article.favoritesCount})</span>
        </S.ActionButton>

        <S.ActionButton disabled={!isAuthor || isLoading} onClick={() => performAction("edit")} className="btn-outline-secondary">
          <Pencil2Icon />
          Edit Article
        </S.ActionButton>

        <S.ActionButton disabled={!isAuthor || isLoading} onClick={() => performAction("delete")} className="btn-outline-danger">
          <TrashIcon />
          Delete Article
        </S.ActionButton>
      </S.ArticleActions>
    </S.ArticleMeta>
  );
};
