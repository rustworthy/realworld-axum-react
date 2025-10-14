import { FC } from "react";

import { AuthorInfo } from "@/entities/Article";
import { UserPayloadUser, useDeleteCommentMutation } from "@/shared/api";
import { ListCommentsApiResponse } from "@/shared/api/generated";
import { ActionButton } from "@/shared/ui/controls/Button";
import { TrashIcon } from "@radix-ui/react-icons";

import * as S from "./CommentList.styles";

export interface ICommentListProps {
  slug: string;
  comments: ListCommentsApiResponse["comments"];
  user: UserPayloadUser["user"] | null;
}

export const CommentList: FC<ICommentListProps> = ({ slug, comments, user }) => {
  const [deleteComment, { isLoading }] = useDeleteCommentMutation();

  return comments.map((comment) => (
    <S.Comment key={comment.id}>
      <S.CommentBody>{comment.body}</S.CommentBody>
      <S.CommentFooter>
        <AuthorInfo imageUrl={comment.author.image} username={comment.author.username} authoredAt={comment.createdAt} />
        {comment.author.username === user?.username ? (
          <ActionButton
            onClick={() => deleteComment({ slug, commentId: comment.id })}
            isDisabled={isLoading}
            className="btn-outline-danger fit"
          >
            <TrashIcon />
          </ActionButton>
        ) : null}
      </S.CommentFooter>
    </S.Comment>
  ));
};
