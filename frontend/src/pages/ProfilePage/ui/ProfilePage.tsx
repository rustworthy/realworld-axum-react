import { useMemo } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router";

import { useAuth } from "@/features/auth";
import { NotFoundPage } from "@/pages/NotFoundPage";
import { useFollowProfileMutation, useListArticlesQuery, useProfileQuery, useUnfollowProfileMutation } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { truncateText } from "@/shared/lib/utils";
import { Preview, PreviewProps } from "@/shared/ui/Article/Preview";
import { Avatar } from "@/shared/ui/Avatar";
import { Pagination, PaginationProps } from "@/shared/ui/Pagination";
import { Tabs } from "@/shared/ui/Tabs";
import { ActionButton } from "@/shared/ui/controls/Button";
import { GearIcon, MinusCircledIcon, PlusCircledIcon } from "@radix-ui/react-icons";

import * as S from "./Profile.styles";

const ARTICLES_PER_PAGE = 3;

export const ProfilePage = () => {
  const { username } = useParams<{ username: string }>();
  if (!username) return <NotFoundPage />;

  const { user: loggedInUser, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  const [searchParams, setSearchParams] = useSearchParams({ feed: "authored", page: "1" });
  const parsedPage = parseInt(searchParams.get("page")!);
  const page = isNaN(parsedPage) || parsedPage < 1 ? 1 : parsedPage;
  const offset = (page - 1) * ARTICLES_PER_PAGE;
  const feed = searchParams.get("feed");
  const isFavoritedView = feed === "favorited";

  const { data, isLoading: isArticlesDataLoading } = useListArticlesQuery({
    limit: ARTICLES_PER_PAGE,
    offset,
    author: isFavoritedView ? undefined : username,
    favorited: isFavoritedView ? username : undefined,
  });
  const pagesCount = useMemo(() => (data ? Math.ceil(data.articlesCount / ARTICLES_PER_PAGE) : null), [data]);
  const empty = !isArticlesDataLoading && (!data || data.articlesCount === 0 || data.articles.length === 0);

  // we normally do not render pagination controls at all if there are no pages
  // or there is a single page (the latter is probably a matter of taste and user
  // feedback); there are edge cases though: imagine the `Favorited Articles` tab
  // is active, there are two pages initially, the user is on the second page and
  // they start revoking the articles; this way they can have the entire `page=2`
  // empty and there will be no control at hand to go to the first (and only) page,
  // unless we cover this case with `(pagesCount === 1 && empty)` check
  const shouldPaginate = typeof pagesCount === "number" && (pagesCount > 1 || (pagesCount === 1 && empty));

  const [followProfile, { isLoading: isFollowLoading }] = useFollowProfileMutation();
  const [unfollowProfile, { isLoading: isUnfollowLoading }] = useUnfollowProfileMutation();
  const { data: profileData, isLoading: isProfileDataLoading } = useProfileQuery({ username });
  if (!profileData) return isProfileDataLoading ? null : <NotFoundPage />;
  const profile = profileData.profile;
  const isProfileOwner = loggedInUser?.username === profile.username;
  const isFollowingProfile = profileData.profile.following;

  const handlePageClick: PaginationProps["onPageChange"] = ({ selected }) => {
    setSearchParams((params) => {
      params.set("page", (selected + 1).toString());
      return params;
    });
    return selected;
  };

  const afterArticleActionCallback: PreviewProps["afterActionCallback"] = (action) => {
    if (action !== "unfavorite") return;
    // this is the last item in the favorited list and they are revoking their
    // "like", so let's navigate them to the previous page (meaning page of the
    // paginated view)
    if (isFavoritedView && data?.articles.length === 1 && page > 1) {
      setSearchParams((params) => {
        params.set("page", (page - 1).toString());
        return params;
      });
    }
  };

  return (
    <S.PageWrapper>
      <S.Banner>
        <S.BannerContainer>
          <Avatar size="lg" imageUrl={profile.image} username={profile.username} />
          <S.ProfileTitle>{profile.username}</S.ProfileTitle>
          <S.ProfileBio>{truncateText(profile.bio, 600)}</S.ProfileBio>
          <S.ProfileActions>
            {isProfileOwner ? (
              <ActionButton
                onClick={() => navigate(ROUTES.SETTINGS)}
                isDisabled={isArticlesDataLoading}
                className="btn-outline-secondary fit"
              >
                <GearIcon />
                Edit Profile Settings
              </ActionButton>
            ) : isAuthenticated && isFollowingProfile ? (
              <ActionButton
                onClick={() => unfollowProfile({ username })}
                isDisabled={isUnfollowLoading}
                className="btn-outline-secondary fit"
              >
                <MinusCircledIcon />
                {`Unfollow ${truncateText(username)}`}
              </ActionButton>
            ) : isAuthenticated && !isFollowingProfile ? (
              <ActionButton
                onClick={() => followProfile({ username })}
                isDisabled={isFollowLoading}
                className="btn-outline-secondary fit"
              >
                <PlusCircledIcon />
                {`Follow ${truncateText(username)}`}
              </ActionButton>
            ) : null}
          </S.ProfileActions>
        </S.BannerContainer>
      </S.Banner>
      <S.MainContent>
        <S.FeedContainer>
          <Tabs.List>
            {/* TODO: figure out why react is unhappy witha the `$isActive` transient prop */}
            <Tabs.Item isActive={!isFavoritedView} to="?feed=authored&page=1" label="My Articles" />
            <Tabs.Item isActive={isFavoritedView} to="?feed=favorited&page=1" label="Favorited Articles" />
          </Tabs.List>
          <S.PreviewList>
            {empty
              ? null
              : isArticlesDataLoading
                ? // TODO: add skeleton while loading
                  null
                : data!.articles.map((article) => (
                    <Preview
                      afterActionCallback={afterArticleActionCallback}
                      actionsEnabled={isAuthenticated}
                      article={article}
                      key={article.slug}
                    />
                  ))}
          </S.PreviewList>
          {shouldPaginate ? <Pagination forcePage={page - 1} onPageChange={handlePageClick} pageCount={pagesCount} /> : null}
        </S.FeedContainer>
      </S.MainContent>
    </S.PageWrapper>
  );
};
