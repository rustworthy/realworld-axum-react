import { FC, useState } from "react";
import { Controller, FieldErrors, useForm } from "react-hook-form";

import { UserPayloadUser, useCreateCommentMutation } from "@/shared/api";
import { AuthorInfo } from "@/shared/ui/Article";
import { Button } from "@/shared/ui/controls/Button";
import { Textarea } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";

import { TCreateCommentSchema, createCommentDefaultValues, createCommentSchema } from "./CommentForm.schema";
import * as S from "./CommentForm.styles";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";
import { ANY_TODO } from "@/shared/types/common.types";
import { toast } from "sonner";

export type ArticleMetaProps = {
  user: UserPayloadUser["user"];
  slug: string;
};

export const CommentForm: FC<ArticleMetaProps> = ({ user, slug }) => {
  const [createComment, { isLoading: isCreateCommentLoading }] = useCreateCommentMutation();

  const [initialErrors, setInitialErrors] = useState<FieldErrors<TCreateCommentSchema> | undefined>(undefined);
  const {
    control,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(createCommentSchema),
    defaultValues: { ...createCommentDefaultValues },
    errors: initialErrors,
  });

  const onSubmit = async (comment: TCreateCommentSchema) => {
    const result = await createComment({ slug, commentPayloadCommentCreate: { comment } });
    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === 422) {
        const validationErrors: Record<keyof TCreateCommentSchema, string[]> = (result.error as ANY_TODO).data.errors;
        const initialErrors: FieldErrors<TCreateCommentSchema> = {};
        for (const [field, errors] of Object.entries(validationErrors)) {
          initialErrors[field as keyof TCreateCommentSchema] = { type: "value", message: errors.join(". ") };
        }
        setInitialErrors(initialErrors);
      }
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Action failed. Please check your internet connection and retry.");
      }
      return;
    }
    reset();
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
