import { FC } from "react";

import { LayoutContainer } from "@/shared/ui/Container";
import { TagList } from "@/shared/ui/TagList";

import * as S from "./HomePage.styles";

export const HomePage: FC = () => {
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
                  <S.TabLink href="">Your Feed</S.TabLink>
                </S.TabItem>
                <S.TabItem>
                  <S.TabLink $isActive href="">
                    Global Feed
                  </S.TabLink>
                </S.TabItem>
              </S.TabList>
            </S.TabContainer>
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
              <a href="/article/how-to-build-webapps-that-scale" className="preview-link">
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
                  <li className="tag-default tag-pill tag-outline">realworld</li>
                  <li className="tag-default tag-pill tag-outline">implementations</li>
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
