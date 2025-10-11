import { FC } from "react";

import type { UserPayloadUser } from "@/shared/api";
import { AuthorInfo } from "@/shared/ui/Article";
import { Button } from "@/shared/ui/controls/Button";

import * as S from "./ArticleComments.styles";

export const ArticleComments: FC<{ user: UserPayloadUser["user"] | null }> = ({ user }) => {
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
    {
      id: 2,
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
  return (
    <>
      {user ? (
        <S.CommentForm>
          <S.CommentFormBody>
            <S.CommentTextarea placeholder="Write a comment..." rows={3} />
          </S.CommentFormBody>
          <S.CommentFormFooter>
            <AuthorInfo imageUrl={user.image} username={user.username} />
            <Button dataTestId="post_comment_button" isDisabled={false}>
              Post Comment
            </Button>
          </S.CommentFormFooter>
        </S.CommentForm>
      ) : null}
      <div>
        {comments.map((comment) => (
          <S.Comment key={comment.id}>
            <S.CommentBody>{comment.body}</S.CommentBody>
            <S.CommentFooter>
              <AuthorInfo imageUrl={comment.author.image} username={comment.author.username} authoredAt={comment.createdAt} />
            </S.CommentFooter>
          </S.Comment>
        ))}
      </div>
    </>
  );
};
