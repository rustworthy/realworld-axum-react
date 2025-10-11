import { FC } from "react";
import { Controller, useForm } from "react-hook-form";

import { UserPayloadUser, useCreateCommentMutation } from "@/shared/api";
import { AuthorInfo } from "@/shared/ui/Article";
import { Button } from "@/shared/ui/controls/Button";
import { Textarea } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";

import { TCreateCommentSchema, createCommentDefaultValues, createCommentSchema } from "./CommentForm.schema";
import * as S from "./CommentForm.styles";

export type ArticleMetaProps = {
  user: UserPayloadUser["user"];
  slug: string;
};

export const CommentForm: FC<ArticleMetaProps> = ({ user, slug }) => {
  const [createComment, { isLoading: isCreateCommentLoading }] = useCreateCommentMutation();

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(createCommentSchema),
    defaultValues: { ...createCommentDefaultValues },
  });

  const onSubmit = async (comment: TCreateCommentSchema) => {
    await createComment({ slug, commentPayloadCommentCreate: { comment } });
  };

  return (
    <S.CommentForm noValidate aria-disabled={isCreateCommentLoading} onSubmit={handleSubmit(onSubmit)}>
      <S.CommentFormBody>
        <Controller
          control={control}
          name="body"
          render={({ field }) => (
            <Textarea
              rows={6}
              field={field}
              required
              id="post_comment_field"
              label="White a comment..."
              error={errors.body?.message}
            />
          )}
        />
      </S.CommentFormBody>
      <S.CommentFormFooter>
        <AuthorInfo imageUrl={user.image} username={user.username} />
        <Button dataTestId="post_comment_button" isDisabled={false}>
          Post Comment
        </Button>
      </S.CommentFormFooter>
    </S.CommentForm>
  );
};
