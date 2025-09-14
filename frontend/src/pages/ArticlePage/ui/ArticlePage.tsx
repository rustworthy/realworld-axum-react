import { FC } from "react";
import { Navigate, useParams } from "react-router";

import { useAuth } from "@/features/auth";

import * as S from "./ArticlePage.styles";
import { useReadArticleQuery } from "@/shared/api/generated";
import { NotFoundPage } from "@/pages/NotFoundPage";

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

export const ArticlePage: FC = () => {
  const { slug } = useParams<{ slug: string }>();
  if (!slug) return <NotFoundPage />;

  const { data } = useReadArticleQuery({ slug: slug! });
  const { user } = useAuth();

  const comments = [
    {
      id: 1,
      createdAt: "2016-02-18T03:22:56.637Z",
      updatedAt: "2016-02-18T03:22:56.637Z",
      body: "With supporting text below as a natural lead-in to additional content.",
      author: {
        username: "jacob-schmidt",
        bio: "I work at statefarm",
        image: "http://i.imgur.com/Qr71crq.jpg",
        following: false,
      },
    },
  ];

  if (!data) return null;

  const article = data.article;
  const isOwner = user?.username === article.author.username;

  return (
    <>
      <S.PageWrapper>
        <S.Banner>
          <S.BannerContainer>
            <S.ArticleTitle>{article.title}</S.ArticleTitle>

            <S.ArticleMeta>
              <S.AuthorInfo>
                <S.AuthorImage src={article.author.image ?? "https://avatars.githubusercontent.com/u/4324516?v=4"} alt={article.author.username} />
                <div>
                  <S.AuthorName href={`/profile/${article.author.username}`}>{article.author.username}</S.AuthorName>
                  <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
                </div>
              </S.AuthorInfo>

              <S.ArticleActions>
                {!isOwner && (
                  <>
                    <S.ActionButton className="btn-outline-secondary">
                      <i className="ion-plus-round" />
                      &nbsp; Follow {article.author.username} <span>(10)</span>
                    </S.ActionButton>
                    <S.ActionButton className="btn-outline-primary">
                      <i className="ion-heart" />
                      &nbsp; Favorite Post <span>({article.favoritesCount})</span>
                    </S.ActionButton>
                  </>
                )}

                {isOwner && (
                  <>
                    <S.ActionButton className="btn-outline-secondary">
                      <i className="ion-edit" /> Edit Article
                    </S.ActionButton>
                    <S.ActionButton className="btn-outline-danger">
                      <i className="ion-trash-a" /> Delete Article
                    </S.ActionButton>
                  </>
                )}
              </S.ArticleActions>
            </S.ArticleMeta>
          </S.BannerContainer>
        </S.Banner>

        <S.MainContent>
          <S.ArticleContent>
            {article.body.split("\n").map((paragraph, index) => {
              if (paragraph.startsWith("## ")) {
                return <h2 key={index}>{paragraph.replace("## ", "")}</h2>;
              }
              if (paragraph.trim()) {
                return <p key={index}>{paragraph}</p>;
              }
              return null;
            })}

            <S.TagList>
              {article.tagList.map((tag) => (
                <S.Tag key={tag}>{tag}</S.Tag>
              ))}
            </S.TagList>
          </S.ArticleContent>

          <S.Separator />

          <S.ArticleMeta>
            <S.AuthorInfo>
              <S.AuthorImage src={article.author.image} alt={article.author.username} />
              <div>
                <S.AuthorName href={`/profile/${article.author.username}`}>{article.author.username}</S.AuthorName>
                <S.ArticleDate>{formatDate(article.createdAt)}</S.ArticleDate>
              </div>
            </S.AuthorInfo>

            <S.ArticleActions>
              {!isOwner && (
                <>
                  <S.ActionButton className="btn-outline-secondary">
                    <i className="ion-plus-round" />
                    &nbsp; Follow {article.author.username}
                  </S.ActionButton>
                  <S.ActionButton className="btn-outline-primary">
                    <i className="ion-heart" />
                    &nbsp; Favorite Article <span>({article.favoritesCount})</span>
                  </S.ActionButton>
                </>
              )}

              {isOwner && (
                <>
                  <S.ActionButton className="btn-outline-secondary">
                    <i className="ion-edit" /> Edit Article
                  </S.ActionButton>
                  <S.ActionButton className="btn-outline-danger">
                    <i className="ion-trash-a" /> Delete Article
                  </S.ActionButton>
                </>
              )}
            </S.ArticleActions>
          </S.ArticleMeta>

          <S.CommentSection>
            {user && (
              <S.CommentForm>
                <S.CommentFormBody>
                  <S.CommentTextarea placeholder="Write a comment..." rows={3} />
                </S.CommentFormBody>
                <S.CommentFormFooter>
                  <S.CommentAuthorImage src={user.image || ""} alt={user.username} />
                  <S.CommentButton type="submit">Post Comment</S.CommentButton>
                </S.CommentFormFooter>
              </S.CommentForm>
            )}

            {comments.map((comment) => (
              <S.Comment key={comment.id}>
                <S.CommentBody>{comment.body}</S.CommentBody>
                <S.CommentFooter>
                  <S.CommentAuthorImage src={comment.author.image} alt={comment.author.username} />
                  <S.CommentAuthor href={`/profile/${comment.author.username}`}>{comment.author.username}</S.CommentAuthor>
                  <S.CommentDate>{formatDate(comment.createdAt)}</S.CommentDate>
                  {user?.username === comment.author.username && (
                    <S.ModOptions>
                      <i className="ion-trash-a" />
                    </S.ModOptions>
                  )}
                </S.CommentFooter>
              </S.Comment>
            ))}
          </S.CommentSection>
        </S.MainContent>
      </S.PageWrapper>
      <div style={{ marginTop: "60px", padding: "20px", backgroundColor: "#f8f9fa", borderTop: "3px solid #333" }}>
        <h3 style={{ textAlign: "center", marginBottom: "30px", color: "#333" }}>
          Original RealWorld Template (for development reference)
        </h3>

        <div className="article-page">
          <div className="banner">
            <div className="container">
              <h1>How to build webapps that scale</h1>

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
                <button className="btn btn-sm btn-outline-secondary">
                  <i className="ion-plus-round" />
                  &nbsp; Follow Eric Simons <span className="counter">(10)</span>
                </button>
                &nbsp;&nbsp;
                <button className="btn btn-sm btn-outline-primary">
                  <i className="ion-heart" />
                  &nbsp; Favorite Post <span className="counter">(29)</span>
                </button>
                <button className="btn btn-sm btn-outline-secondary">
                  <i className="ion-edit" /> Edit Article
                </button>
                <button className="btn btn-sm btn-outline-danger">
                  <i className="ion-trash-a" /> Delete Article
                </button>
              </div>
            </div>
          </div>

          <div className="container page">
            <div className="row article-content">
              <div className="col-md-12">
                <p>Web development technologies have evolved at an incredible clip over the past few years.</p>
                <h2 id="introducing-ionic">Introducing RealWorld.</h2>
                <p>It's a great solution for learning how other frameworks work.</p>
                <ul className="tag-list">
                  <li className="tag-default tag-pill tag-outline">realworld</li>
                  <li className="tag-default tag-pill tag-outline">implementations</li>
                </ul>
              </div>
            </div>

            <hr />

            <div className="article-actions">
              <div className="article-meta">
                <a href="profile.html">
                  <img src="http://i.imgur.com/Qr71crq.jpg" />
                </a>
                <div className="info">
                  <a href="" className="author">
                    Eric Simons
                  </a>
                  <span className="date">January 20th</span>
                </div>
                <button className="btn btn-sm btn-outline-secondary">
                  <i className="ion-plus-round" />
                  &nbsp; Follow Eric Simons
                </button>
                &nbsp;
                <button className="btn btn-sm btn-outline-primary">
                  <i className="ion-heart" />
                  &nbsp; Favorite Article <span className="counter">(29)</span>
                </button>
                <button className="btn btn-sm btn-outline-secondary">
                  <i className="ion-edit" /> Edit Article
                </button>
                <button className="btn btn-sm btn-outline-danger">
                  <i className="ion-trash-a" /> Delete Article
                </button>
              </div>
            </div>

            <div className="row">
              <div className="col-xs-12 col-md-8 offset-md-2">
                <form className="card comment-form">
                  <div className="card-block">
                    <textarea className="form-control" placeholder="Write a comment..." rows={3} />
                  </div>
                  <div className="card-footer">
                    <img src="http://i.imgur.com/Qr71crq.jpg" className="comment-author-img" />
                    <button className="btn btn-sm btn-primary">Post Comment</button>
                  </div>
                </form>

                <div className="card">
                  <div className="card-block">
                    <p className="card-text">With supporting text below as a natural lead-in to additional content.</p>
                  </div>
                  <div className="card-footer">
                    <a href="/profile/author" className="comment-author">
                      <img src="http://i.imgur.com/Qr71crq.jpg" className="comment-author-img" />
                    </a>
                    &nbsp;
                    <a href="/profile/jacob-schmidt" className="comment-author">
                      Jacob Schmidt
                    </a>
                    <span className="date-posted">Dec 29th</span>
                  </div>
                </div>

                <div className="card">
                  <div className="card-block">
                    <p className="card-text">With supporting text below as a natural lead-in to additional content.</p>
                  </div>
                  <div className="card-footer">
                    <a href="/profile/author" className="comment-author">
                      <img src="http://i.imgur.com/Qr71crq.jpg" className="comment-author-img" />
                    </a>
                    &nbsp;
                    <a href="/profile/jacob-schmidt" className="comment-author">
                      Jacob Schmidt
                    </a>
                    <span className="date-posted">Dec 29th</span>
                    <span className="mod-options">
                      <i className="ion-trash-a" />
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};
