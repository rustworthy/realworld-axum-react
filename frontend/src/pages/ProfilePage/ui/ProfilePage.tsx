import { useMemo } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";

import { useAuth } from "@/features/auth";
import { NotFoundPage } from "@/pages/NotFoundPage";
import { useListArticlesQuery, useReadCurrentUserQuery } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { truncateText } from "@/shared/lib/utils";
import { Preview } from "@/shared/ui/Article/Preview";
import { Avatar } from "@/shared/ui/Avatar";
import { Pagination, PaginationProps } from "@/shared/ui/Pagination";
import { Tabs } from "@/shared/ui/Tabs";
import { ActionButton } from "@/shared/ui/controls/Button";
import { GearIcon, PlusCircledIcon } from "@radix-ui/react-icons";

import * as S from "./Profile.styles";

const ARTICLES_PER_PAGE = 3;

export const ProfilePage = () => {
  const { username } = useParams<{ username: string }>();
  if (!username) return <NotFoundPage />;

  const { user: loggedInUser, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  const [searchParams, setSearchParams] = useSearchParams({ feed: "global", page: "1" });
  const parsedPage = parseInt(searchParams.get("page")!);
  const page = isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage;
  const offset = (page - 1) * ARTICLES_PER_PAGE;

  const { data, isLoading } = useListArticlesQuery({ author: username, limit: ARTICLES_PER_PAGE, offset });
  const pagesCount = useMemo(() => (data ? Math.ceil(data.articlesCount / ARTICLES_PER_PAGE) : null), [data]);
  const empty = !isLoading && (!data || data.articlesCount === 0 || data.articles.length === 0);
  const shouldPaginate = typeof pagesCount === "number" && pagesCount > 1 && !empty;

  const { data: profileData, isLoading: isProfileDataLoading } = useReadCurrentUserQuery();
  if (!profileData) return isProfileDataLoading ? null : <NotFoundPage />;
  const user = profileData.user;
  const isProfileOwner = loggedInUser?.username === user.username;

  const handlePageClick: PaginationProps["onPageChange"] = ({ selected }) => {
    setSearchParams((params) => {
      params.set("page", (selected + 1).toString());
      return params;
    });
    return selected;
  };

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <Avatar size="lg" imageUrl={user.image} username={user.username} />
          <S.ProfileTitle>{user.username}</S.ProfileTitle>
          <S.ProfileBio>{truncateText(user.bio, 600)}</S.ProfileBio>
          <S.ProfileActions>
            {isProfileOwner ? (
              <ActionButton
                onClick={() => navigate(ROUTES.SETTINGS)}
                isDisabled={isLoading}
                className="btn-outline-secondary fit"
              >
                <GearIcon />
                Edit Profile Settings
              </ActionButton>
            ) : (
              <ActionButton isDisabled={isLoading} className="btn-outline-secondary fit">
                <PlusCircledIcon />
                {`Follow ${truncateText(username)}`}
              </ActionButton>
            )}
          </S.ProfileActions>
        </S.BannerContainer>
      </S.Banner>
      <S.MainContent>
        <S.FeedContainer>
          <Tabs.Root>
            <Tabs.List>
              <Tabs.Item>
                <Tabs.Link to="">My Articles</Tabs.Link>
              </Tabs.Item>
              <Tabs.Item>
                {/* TODO: figure out why react is unhappy witha the `$isActive` transient prop */}
                <Tabs.Link to="">Favorited Articles</Tabs.Link>
              </Tabs.Item>
            </Tabs.List>
          </Tabs.Root>
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
      </S.MainContent>
    </S.PageWrapper>
  );
};
