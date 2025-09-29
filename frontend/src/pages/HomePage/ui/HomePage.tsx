import { FC, useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router";

import { useAuth } from "@/features/auth";
import { useListArticlesQuery, usePersonalFeedQuery } from "@/shared/api";
import { TagList } from "@/shared/ui/Article";
import { Preview } from "@/shared/ui/Article/Preview";
import { LayoutContainer } from "@/shared/ui/Container";
import { Pagination, type PaginationProps } from "@/shared/ui/Pagination";

import * as S from "./HomePage.styles";

const ARTICLES_PER_PAGE = 5;

export type FeedType = "personal" | "global";

export type FeedSearchParams = {
  feed?: FeedType;
};

export const HomePage: FC = () => {
  const { isAuthenticated } = useAuth();

  const [searchParams, setSearchParams] = useSearchParams({ feed: "global", page: "1" });
  const parsedPage = parseInt(searchParams.get("page")!);
  const page = isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage;
  const offset = (page - 1) * ARTICLES_PER_PAGE;
  const tag = searchParams.get("tag");
  const isTagView = tag !== null;
  const isPersonalFeed = searchParams.get("feed") === "personal" && !isTagView;
  const useArticlesList = isPersonalFeed ? usePersonalFeedQuery : useListArticlesQuery;
  const { data, isLoading } = useArticlesList({ limit: ARTICLES_PER_PAGE, offset, tag: tag ?? undefined });
  const pagesCount = useMemo(() => (data ? Math.ceil(data.articlesCount / ARTICLES_PER_PAGE) : null), [data]);
  // it's possible that url contains page that is past the aricles: e.g. they might
  // have manullay inserted the parameter (which is less likely) or the number of
  // articles decreased prior to them refreshing the page, and so there _are_ articles,
  // but not at this offset; so we need to check both `artcilesCount` and `articles.length`;
  // note that we prefer not to render pagination controls, if there is only one page
  const empty = !isLoading && (!data || data.articlesCount === 0 || data.articles.length === 0);
  const shouldPaginate = typeof pagesCount === "number" && pagesCount > 1 && !empty;

  const [selectedTag, setSelectedTag] = useState<string | null>(null);

  const handlePageClick: PaginationProps["onPageChange"] = ({ selected }) => {
    setSearchParams((params) => {
      params.set("page", (selected + 1).toString());
      return params;
    });
    return selected;
  };

  const handleTagClick = (tag: string) => {
    setSelectedTag(tag);
    setSearchParams((params) => {
      params.set("tag", tag);
      params.set("page", "1");
      return params;
    });
  };

  // say they fefresh the page or open a link someone shared with them:
  // if the tag is in the serach params we are treating it as selected
  // and activated
  useEffect(() => {
    if (tag !== null) {
      setSelectedTag(tag);
    }
  }, []);

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <S.BannerTitle>conduit</S.BannerTitle>
          <S.BannerDescription>A place to share your knowledge.</S.BannerDescription>
        </S.BannerContainer>
      </S.Banner>

      <LayoutContainer>
        <S.MainContent>
          <S.FeedContainer>
            <S.TabContainer>
              <S.TabList>
                <S.TabItem>
                  <S.TabLink $isActive={isPersonalFeed} to="/?feed=personal&page=1">
                    Your Feed
                  </S.TabLink>
                </S.TabItem>
                <S.TabItem>
                  {/* TODO: figure out why react is unhappy witha the `$isActive` transient prop */}
                  <S.TabLink $isActive={!isPersonalFeed && !isTagView} to="/?feed=global&page=1">
                    Global Feed
                  </S.TabLink>
                </S.TabItem>
                {selectedTag ? (
                  <S.TabItem>
                    <S.TabLink $isActive={isTagView} to={`/?tag=${selectedTag}`}>
                      {selectedTag}
                    </S.TabLink>
                  </S.TabItem>
                ) : null}
              </S.TabList>
            </S.TabContainer>
            <S.PreviewList>
              {empty
                ? null
                : isLoading
                  ? // TODO: add skeleton while loading
                    null
                  : data!.articles.map((article) => (
                      <Preview actionsEnabled={isAuthenticated} article={article} key={article.slug} />
                    ))}
            </S.PreviewList>
            {shouldPaginate ? <Pagination forcePage={page - 1} onPageChange={handlePageClick} pageCount={pagesCount} /> : null}
          </S.FeedContainer>
          <S.TagsContainer>
            <p>Popular tags</p>
            <TagList onClick={handleTagClick} tags={["art", "programming", "react", "rust"]} />
          </S.TagsContainer>
        </S.MainContent>
      </LayoutContainer>
    </S.PageWrapper>
  );
};
