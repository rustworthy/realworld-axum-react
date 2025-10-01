import { FC, useCallback } from "react";
import { Link } from "react-router";

import { ArticlePayloadArticle, useFavoriteArticleMutation, useUnfavoriteArticleMutation } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { formatEventsCount, parseOutErrorMessage, truncateText } from "@/shared/lib/utils";
import { HeartFilledIcon, HeartIcon } from "@radix-ui/react-icons";
import { toast } from "sonner";

import { ActionButton } from "../controls/Button";
import { AuthorInfo } from "./AuthorInfo";
import * as S from "./Preview.styles";
import { TagList } from "./TagList";

export type PreviewProps = {
  article: ArticlePayloadArticle["article"];
  actionsEnabled?: boolean;
  afterActionCallback?: (action: string) => void;
};

export const Preview: FC<PreviewProps> = ({ article, actionsEnabled, afterActionCallback }) => {
  const [favArticle, { isLoading: isFavLoading }] = useFavoriteArticleMutation();
  const [unfavArticle, { isLoading: isUnfavLoading }] = useUnfavoriteArticleMutation();
  const isLoading = isFavLoading || isUnfavLoading;

  const performAction = useCallback(
    async (action: string) => {
      switch (action) {
        case "favorite": {
          const result = await favArticle({ slug: article.slug });
          if (result.error) {
            const msg = parseOutErrorMessage(result.error);
            toast.error(msg);
          }
          afterActionCallback?.(action);
          return;
        }
        case "unfavorite": {
          const result = await unfavArticle({ slug: article.slug });
          if (result.error) {
            const msg = parseOutErrorMessage(result.error);
            toast.error(msg);
          }
          afterActionCallback?.(action);
          return;
        }
        default:
          throw new Error("Unsupported action");
      }
    },
    [article],
  );

  return (
    <S.PreviewContainer>
      <S.PreviewMeta>
        <AuthorInfo article={article} />
        {!actionsEnabled ? null : article.favorited ? (
          <ActionButton onClick={() => performAction("unfavorite")} isDisabled={isLoading} className="btn-outline-primary fit">
            <HeartFilledIcon />
            <span>{formatEventsCount(article.favoritesCount)}</span>
          </ActionButton>
        ) : (
          <ActionButton onClick={() => performAction("favorite")} isDisabled={isLoading} className="btn-outline-primary fit">
            <HeartIcon />
            <span>{article.favoritesCount}</span>
          </ActionButton>
        )}
      </S.PreviewMeta>

      <Link title="Read full article" to={`${ROUTES.ARTICLE}/${article.slug}`}>
        <S.PreviewTitle>{truncateText(article.title, 50)}</S.PreviewTitle>
        <S.PreviewDescription>{truncateText(article.description, 150)}</S.PreviewDescription>
      </Link>
      <S.PreviewFooter>
        <S.ReadMoreLink title="Read full article" to={`${ROUTES.ARTICLE}/${article.slug}`}>
          Read more...
        </S.ReadMoreLink>
        <TagList tagClassName="outline" tags={article.tagList} />
      </S.PreviewFooter>
    </S.PreviewContainer>
  );
};
