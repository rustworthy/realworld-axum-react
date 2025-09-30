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
import { ActionButton } from "@/shared/ui/controls/Button";
import { GearIcon, PlusCircledIcon } from "@radix-ui/react-icons";

import * as S from "./Profile.styles";

const ARTICLES_PER_PAGE = 5;

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
        <div className="profile-page">
          <div className="container">
            <div className="row">
              <div className="col-xs-12 col-md-10 offset-md-1">
                <div className="articles-toggle">
                  <ul className="nav nav-pills outline-active">
                    <li className="nav-item">
                      <a className="nav-link active" href="">
                        My Articles
                      </a>
                    </li>
                    <li className="nav-item">
                      <a className="nav-link" href="">
                        Favorited Articles
                      </a>
                    </li>
                  </ul>
                </div>

                <div className="article-preview">
                  <div className="article-meta">
                    <a href="/profile/eric-simons">
                      <img src="http://i.imgur.com/Qr71crq.jpg" />
                    </a>
                    <div className="info">
                      <a href="/profile/eric-simons" className="author">
                        Eric Simons
                      </a>
                      <span className="date">January 20th</span>
                    </div>
                    <button className="btn btn-outline-primary btn-sm pull-xs-right">
                      <i className="ion-heart" /> 29
                    </button>
                  </div>
                  <a href="/article/how-to-buil-webapps-that-scale" className="preview-link">
                    <h1>How to build webapps that scale</h1>
                    <p>This is the description for the post.</p>
                    <span>Read more...</span>
                    <ul className="tag-list">
                      <li className="tag-default tag-pill tag-outline">realworld</li>
                      <li className="tag-default tag-pill tag-outline">implementations</li>
                    </ul>
                  </a>
                </div>

                <div className="article-preview">
                  <div className="article-meta">
                    <a href="/profile/albert-pai">
                      <img src="http://i.imgur.com/N4VcUeJ.jpg" />
                    </a>
                    <div className="info">
                      <a href="/profile/albert-pai" className="author">
                        Albert Pai
                      </a>
                      <span className="date">January 20th</span>
                    </div>
                    <button className="btn btn-outline-primary btn-sm pull-xs-right">
                      <i className="ion-heart" /> 32
                    </button>
                  </div>
                  <a href="/article/the-song-you" className="preview-link">
                    <h1>The song you won't ever stop singing. No matter how hard you try.</h1>
                    <p>This is the description for the post.</p>
                    <span>Read more...</span>
                    <ul className="tag-list">
                      <li className="tag-default tag-pill tag-outline">Music</li>
                      <li className="tag-default tag-pill tag-outline">Song</li>
                    </ul>
                  </a>
                </div>

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
              </div>
            </div>
          </div>
        </div>
      </S.MainContent>
    </S.PageWrapper>
  );
};
