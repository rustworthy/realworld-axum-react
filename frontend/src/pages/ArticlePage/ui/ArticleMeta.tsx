import { FC, useCallback } from "react";
import { useNavigate } from "react-router";

import {
  useDeleteArticleMutation,
  useFavoriteArticleMutation,
  useFollowProfileMutation,
  useUnfavoriteArticleMutation,
  useUnfollowProfileMutation,
} from "@/shared/api";
import type { ArticlePayloadArticle, UserPayloadUser } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { formatCount, parseOutErrorMessage, truncateText } from "@/shared/lib/utils";
import { AuthorInfo } from "@/shared/ui/Article";
import { ActionButton } from "@/shared/ui/controls/Button";
import { HeartFilledIcon, HeartIcon, Pencil2Icon, PlusCircledIcon, TrashIcon } from "@radix-ui/react-icons";
import { toast } from "sonner";

import * as S from "./ArticleMeta.styles";

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
  const [followProfile, { isLoading: isFollowLoading }] = useFollowProfileMutation();
  const [unfollowProfile, { isLoading: isUnfollowLoading }] = useUnfollowProfileMutation();

  const authorUsername = article.author.username;
  const isLoading = isDeleteLoading || isFavLoading || isUnfavLoading;
  const isAuthor = authorUsername === user?.username;
  const isFollowing = article.author.following;

  const performAction = useCallback(
    async (action: string) => {
      switch (action) {
        case "follow": {
          await followProfile({ username: authorUsername });
          return;
        }
        case "unfollow": {
          await unfollowProfile({ username: authorUsername });
          return;
        }
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
      <AuthorInfo imageUrl={article.author.image} username={article.author.username} authoredAt={article.createdAt} />

      {user ? (
        <S.ArticleActions>
          {/* -----------------------  follow/unfollow --------------------- */}
          {!isAuthor && isFollowing ? (
            <ActionButton
              onClick={() => performAction("unfollow")}
              isDisabled={isUnfollowLoading}
              className="btn-outline-secondary"
            >
              <PlusCircledIcon />
              {`Unfollow ${truncateText(authorUsername, 10)}`}
            </ActionButton>
          ) : !isAuthor && !isFollowing ? (
            <ActionButton onClick={() => performAction("follow")} isDisabled={isFollowLoading} className="btn-outline-secondary">
              <PlusCircledIcon />
              {`Follow ${truncateText(authorUsername, 12)}`}
            </ActionButton>
          ) : null}

          {/* --------------------- favorite/unfavorite -------------------- */}
          {article.favorited ? (
            <ActionButton onClick={() => performAction("unfavorite")} isDisabled={isLoading} className="btn-outline-primary">
              <HeartFilledIcon />
              Unfavorite Article <span>({formatCount(article.favoritesCount)})</span>
            </ActionButton>
          ) : (
            <ActionButton onClick={() => performAction("favorite")} isDisabled={isLoading} className="btn-outline-primary">
              <HeartIcon />
              Favorite Article <span>({article.favoritesCount})</span>
            </ActionButton>
          )}

          {/* ------------------------- edit/delete ------------------------ */}
          {isAuthor ? (
            <>
              <ActionButton
                isDisabled={!isAuthor || isLoading}
                onClick={() => performAction("edit")}
                className="btn-outline-secondary compact"
              >
                <Pencil2Icon />
                Edit Article
              </ActionButton>

              <ActionButton isDisabled={isLoading} onClick={() => performAction("delete")} className="btn-outline-danger compact">
                <TrashIcon />
                Delete Article
              </ActionButton>
            </>
          ) : null}
        </S.ArticleActions>
      ) : null}
    </S.ArticleMeta>
  );
};
