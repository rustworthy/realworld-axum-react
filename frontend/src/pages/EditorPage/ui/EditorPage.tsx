import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useCreateArticleMutation } from "@/shared/api/generated";
import { ROUTES } from "@/shared/constants/routes.constants";
import { ANY_TODO } from "@/shared/types/common.types";
import { FormPage } from "@/shared/ui/FormPage";
import { Button } from "@/shared/ui/controls/Button";
import { EditorInput, TextInput } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TEditorPageSchema, editorPageDefaultValues, editorPageSchema } from "./EditorPage.schema";
import * as S from "./EditorPage.styles";

export const EditorPage = () => {
  const navigate = useNavigate();
  const [create, { isLoading }] = useCreateArticleMutation();

  const {
    control,
    setValue,
    watch,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(editorPageSchema),
    defaultValues: { ...editorPageDefaultValues },
  });

  const onSubmit = async (data: TEditorPageSchema): Promise<void> => {
    const result = await create({
      articlePayloadArticleCreate: {
        article: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === 422) {
        // TODO: think about how to simplify extracting error messages
        const fieldType = Object.keys((result.error as ANY_TODO).data?.errors)[0];

        toast.error(`Failed to register. Reason: ${(result.error as ANY_TODO).data?.errors?.[fieldType]?.[0]}`);
      }
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Failed to register. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Success! Your article has been published.");
    navigate(`${ROUTES.ARTICLE}/${result.data.article.slug}`);
  };

  return (
    <FormPage.Container title="New Article">
      <S.EditorForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isLoading}>
        <Controller
          control={control}
          name="title"
          render={({ field }) => (
            <TextInput autoFocus field={field} required id="editor_title" label="Article's title" error={errors.title?.message} />
          )}
        />

        <Controller
          control={control}
          name="description"
          render={({ field }) => (
            <TextInput
              field={field}
              required
              id="editor_description"
              label="What's this article about?"
              error={errors.description?.message}
            />
          )}
        />

        <EditorInput
          value={watch("body") as string}
          onChange={(value) => setValue("body", value)}
          error={errors.body?.message}
          label="Write your article (in markdown)"
          required
          id="editor_body"
          name="body"
        />

        <Controller
          control={control}
          name="tagList"
          render={({ field }) => (
            <TextInput
              field={field}
              required
              id="editor_tags"
              label="Enter tags (comma separated)"
              error={errors.tagList?.message}
            />
          )}
        />

        <S.SubmitButtonContainer>
          <Button dataTestId="editor_submit_button" isDisabled={isLoading}>
            Publish Article
          </Button>
        </S.SubmitButtonContainer>
      </S.EditorForm>
    </FormPage.Container>
  );
};
