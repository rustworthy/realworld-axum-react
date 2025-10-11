import { FC } from "react";

import type { UserPayloadUser } from "@/shared/api";
import { formatDate } from "@/shared/lib/utils";

import * as S from "./ArticleComments.styles";
import { Avatar } from "@/shared/ui/Avatar";
import { Button } from "@/shared/ui/controls/Button";

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
  ];
  return (
    <S.CommentSection>
      {user && (
        <S.CommentForm>
          <S.CommentFormBody>
            <S.CommentTextarea placeholder="Write a comment..." rows={3} />
          </S.CommentFormBody>
          <S.CommentFormFooter>
            <Avatar size="sm" imageUrl={user.image} username={user.username} />
            <Button dataTestId="post_comment_button" isDisabled={false}>
              Post Comment
            </Button>
          </S.CommentFormFooter>
        </S.CommentForm>
      )}

      {comments.map((comment) => (
        <S.Comment key={comment.id}>
          <S.CommentBody>{comment.body}</S.CommentBody>
          <S.CommentFooter>
            <Avatar size="sm" imageUrl={comment.author.image} username={comment.author.username} />
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
  );
};
