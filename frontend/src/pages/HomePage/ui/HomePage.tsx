import { FC } from "react";
import { useSearchParams } from "react-router";

import { useAuth } from "@/features/auth";
import { useListArticlesQuery } from "@/shared/api";
import { TagList } from "@/shared/ui/Article";
import { Preview } from "@/shared/ui/Article/Preview";
import { LayoutContainer } from "@/shared/ui/Container";

import * as S from "./HomePage.styles";

const IS_PAGINATION_FEATURE_FINISHED = false;

export type FeedType = "personal" | "global";

export type FeedSearchParams = {
  feed?: FeedType;
};

export const HomePage: FC = () => {
  const { isAuthenticated } = useAuth();
  const [searchParams, _setSearchParams] = useSearchParams({ feed: "global" });
  const isPersonalFeed = searchParams.get("feed") === "personal";
  const { data, isLoading } = useListArticlesQuery({ limit: 10 });
  const empty = !isLoading && (!data || data.articlesCount === 0);

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
            {IS_PAGINATION_FEATURE_FINISHED ? (
              <ul className="pagination">
                <li className="page-item active">
                  <a className="page-link" href="">
                    1
                  </a>
                </li>
                <li className="page-item">
                  <a className="page-link" href="">
                    2
                  </a>
                </li>
              </ul>
            ) : null}
          </S.FeedContainer>
          <S.TagsContainer>
            <p>Popular tags</p>
            <TagList tags={["art", "programming", "react", "rust"]} />
          </S.TagsContainer>
        </S.MainContent>
      </LayoutContainer>
    </S.PageWrapper>
  );
};
