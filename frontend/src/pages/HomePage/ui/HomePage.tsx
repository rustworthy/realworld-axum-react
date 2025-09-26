import { FC, useMemo } from "react";
import { useSearchParams } from "react-router";

import { useAuth } from "@/features/auth";
import { useListArticlesQuery } from "@/shared/api";
import { TagList } from "@/shared/ui/Article";
import { Preview } from "@/shared/ui/Article/Preview";
import { LayoutContainer } from "@/shared/ui/Container";

import * as S from "./HomePage.styles";


const ARTICLES_PER_PAGE = 4;

export type FeedType = "personal" | "global";

export type FeedSearchParams = {
  feed?: FeedType;
};

export const HomePage: FC = () => {
  const { isAuthenticated } = useAuth();

  const [searchParams, setSearchParams] = useSearchParams({ feed: "global", page: "1" });
  const isPersonalFeed = searchParams.get("feed") === "personal";
  const parsedPage = parseInt(searchParams.get("page")!);
  const page = isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage;
  const offset = (page - 1) * ARTICLES_PER_PAGE;
  const { data, isLoading } = useListArticlesQuery({ limit: ARTICLES_PER_PAGE, offset });
  const pagesCount = useMemo(() => data ? Math.ceil(data.articlesCount / ARTICLES_PER_PAGE) : null, [data]);
  // it's possible that url contains page that is past the aricles: e.g. they might
  // have manullay inserted the parameter (which is less likely) or the number of
  // articles decreased prior to them refreshing the page, and so there _are_ articles,
  // but not at thos offset; so we need to check both `artcilesCount` and `articles.length`
  const empty = !isLoading && (!data || data.articlesCount === 0 || data.articles.length === 0);
  const shouldPaginate = typeof pagesCount === "number" && pagesCount > 0 && !empty;

  const onPageButtonClick = (page: number) => {
    setSearchParams((params) => {
      params.set("page", page.toString());
      return params;
    });
  }

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
                  <S.TabLink $isActive={isPersonalFeed} to="/?feed=personal">
                    Your Feed
                  </S.TabLink>
                </S.TabItem>
                <S.TabItem>
                  <S.TabLink $isActive={!isPersonalFeed} to="/?feed=global">
                    Global Feed
                  </S.TabLink>
                </S.TabItem>
              </S.TabList>
            </S.TabContainer>
            {empty
              ? null
              : isLoading
                ? // TODO: add skeleton while loading
                null
                : data!.articles.map((article) => (
                  <Preview actionsEnabled={isAuthenticated} article={article} key={article.slug} />
                ))}
            {shouldPaginate ? (
              <ul className="pagination">
                {
                  Array(pagesCount).keys().map(idx => (
                    <li key={idx} className="page-item active">
                      <button onClick={() => onPageButtonClick(idx + 1)}>
                        {idx + 1}
                      </button>
                    </li>
                  ))
                }
              </ul>
            ) : null}
          </S.FeedContainer>
          <S.TagsContainer>
            <p>Popular tags</p>
            <TagList tags={["art", "programming", "react", "rust"]} />
          </S.TagsContainer>
        </S.MainContent>
      </LayoutContainer>
    </S.PageWrapper >
  );
};
