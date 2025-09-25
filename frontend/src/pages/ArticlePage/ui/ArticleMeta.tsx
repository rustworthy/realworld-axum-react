import { FC, useCallback } from "react";
import { useNavigate } from "react-router";

import { useDeleteArticleMutation, useFavoriteArticleMutation, useUnfavoriteArticleMutation } from "@/shared/api";
import type { ArticlePayloadArticle, UserPayloadUser } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { formatDate, formatEventsCount, formatUsername, parseOutErrorMessage } from "@/shared/lib/utils";
import { HeartFilledIcon, HeartIcon, Pencil2Icon, PlusCircledIcon, TrashIcon } from "@radix-ui/react-icons";
import { toast } from "sonner";

import * as S from "./ArticlePage.styles";

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
  const [favArticle, { isLoading: isFavLoading }] = useFavoriteArticleMutation();
  const [unfavArticle, { isLoading: isUnfavLoading }] = useUnfavoriteArticleMutation();

  const authorUsername = article.author.username;
  const profilePath = `/profile/${authorUsername}`;
  const isLoading = isDeleteLoading || isFavLoading || isUnfavLoading;
  const isAuthor = authorUsername === user?.username;

  const performAction = useCallback(
    async (action: string) => {
      switch (action) {
        case "favorite": {
          const result = await favArticle({ slug: article.slug });
          if (result.error) {
            const msg = parseOutErrorMessage(result.error);
            toast.error(msg);
          }
          return;
        }
        case "unfavorite": {
          const result = await unfavArticle({ slug: article.slug });
          if (result.error) {
            const msg = parseOutErrorMessage(result.error);
            toast.error(msg);
          }
          return;
        }
        case "edit": {
          navigate(`${ROUTES.EDITOR}/${article.slug}`);
          return;
        }
        case "delete": {
          const result = await deleteArticle({ slug: article.slug });
          if (result.error) {
            const msg = parseOutErrorMessage(result.error);
            toast.error(msg);
            return;
          }
          toast.success("Your article has been delete.");
          navigate(ROUTES.EDITOR);
          return;
        }
        default:
          throw new Error("Unsupported action");
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
            alt={`${authorUsername}'s profile picture`}
          />
        </a>
        <S.AuthorInfoNameBlock>
          <S.AuthorName href={profilePath}>{formatUsername(authorUsername, 20)}</S.AuthorName>
          <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
        </S.AuthorInfoNameBlock>
      </S.AuthorInfo>

      {user ? (
        <S.ArticleActions>
          {/* -----------------------  follow/unfollow -------------------- */}
          <S.ActionButton disabled={isAuthor || isLoading} className="btn-outline-secondary">
            <PlusCircledIcon />
            {`Follow ${formatUsername(authorUsername)}`}
          </S.ActionButton>

          {/* --------------------- favorite/unfavorite ------------------- */}
          {article.favorited ? (
            <S.ActionButton onClick={() => performAction("unfavorite")} disabled={isLoading} className="btn-outline-primary">
              <HeartFilledIcon />
              Unfavorite Article <span>({formatEventsCount(article.favoritesCount)})</span>
            </S.ActionButton>
          ) : (
            <S.ActionButton onClick={() => performAction("favorite")} disabled={isLoading} className="btn-outline-primary">
              <HeartIcon />
              Favorite Article <span>({article.favoritesCount})</span>
            </S.ActionButton>
          )}

          {/* --------------------------- edit --------------------------- */}
          <S.ActionButton
            disabled={!isAuthor || isLoading}
            onClick={() => performAction("edit")}
            className="btn-outline-secondary compact"
          >
            <Pencil2Icon />
            Edit Article
          </S.ActionButton>

          {/* -------------------------- delete -------------------------- */}
          <S.ActionButton
            disabled={!isAuthor || isLoading}
            onClick={() => performAction("delete")}
            className="btn-outline-danger compact"
          >
            <TrashIcon />
            Delete Article
          </S.ActionButton>
        </S.ArticleActions>
      ) : null}
    </S.ArticleMeta>
  );
};
