import { FC, useCallback } from "react";
import { useNavigate } from "react-router";

import { useDeleteArticleMutation, useFavoriteArticleMutation, useUnfavoriteArticleMutation } from "@/shared/api";
import type { ArticlePayloadArticle, UserPayloadUser } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { formatEventsCount, parseOutErrorMessage, truncateText } from "@/shared/lib/utils";
import { AuthorInfo } from "@/shared/ui/Article";
import { ActionButton } from "@/shared/ui/controls/Button";
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
          navigate(ROUTES.HOME);
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
      <AuthorInfo article={article} />

      {user ? (
        <S.ArticleActions>
          {/* -----------------------  follow/unfollow -------------------- */}

          <ActionButton isDisabled={isAuthor || isLoading} className="btn-outline-secondary">
            <PlusCircledIcon />
            {`Follow ${truncateText(authorUsername)}`}
          </ActionButton>

          {/* --------------------- favorite/unfavorite ------------------- */}
          {article.favorited ? (
            <ActionButton onClick={() => performAction("unfavorite")} isDisabled={isLoading} className="btn-outline-primary">
              <HeartFilledIcon />
              Unfavorite Article <span>({formatEventsCount(article.favoritesCount)})</span>
            </ActionButton>
          ) : (
            <ActionButton onClick={() => performAction("favorite")} isDisabled={isLoading} className="btn-outline-primary">
              <HeartIcon />
              Favorite Article <span>({article.favoritesCount})</span>
            </ActionButton>
          )}

          {/* --------------------------- edit --------------------------- */}
          <ActionButton
            isDisabled={!isAuthor || isLoading}
            onClick={() => performAction("edit")}
            className="btn-outline-secondary compact"
          >
            <Pencil2Icon />
            Edit Article
          </ActionButton>

          {/* -------------------------- delete -------------------------- */}
          <ActionButton
            isDisabled={!isAuthor || isLoading}
            onClick={() => performAction("delete")}
            className="btn-outline-danger compact"
          >
            <TrashIcon />
            Delete Article
          </ActionButton>
        </S.ArticleActions>
      ) : null}
    </S.ArticleMeta>
  );
};
