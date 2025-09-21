import { FC } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { NotFoundPage } from "@/pages/NotFoundPage";
import { useCreateArticleMutation, useReadArticleQuery } from "@/shared/api/generated";
import { ROUTES } from "@/shared/constants/routes.constants";
import { ANY_TODO } from "@/shared/types/common.types";
import { FormPage } from "@/shared/ui/FormPage";
import { Button } from "@/shared/ui/controls/Button";
import { EditorInput, TextInput } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TEditorPageSchema, editorPageDefaultValues, editorPageSchema } from "./EditorPage.schema";
import * as S from "./EditorPage.styles";

const CreateArcticle = () => {
  const navigate = useNavigate();
  const [create, { isLoading }] = useCreateArticleMutation();

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

        toast.error(`Action failed. Reason: ${(result.error as ANY_TODO).data?.errors?.[fieldType]?.[0]}`);
      }
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Action failed. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Success! Your article has been published.");
    navigate(`${ROUTES.ARTICLE}/${result.data.article.slug}`);
  };

  return (
    <FormPage.Container title="New Article">
      <EditorForm onSubmit={onSubmit} disabled={isLoading} />
    </FormPage.Container>
  );
};

const UpdateArticle = () => {
  const { slug } = useParams();
  // if the `slug` is not in path params, we are rending create
  // article view and so we are expecting slug to be there
  const { data, isLoading } = useReadArticleQuery({ slug: slug! });

  // meaning the fetch operation finished and no data returned
  // from the server; apparently, the slug is wrong and the article
  // has been deleted
  if (!isLoading && !data) return <NotFoundPage />;

  const navigate = useNavigate();
  const [create, { isLoading: isCreateArticleLoading }] = useCreateArticleMutation(); // TODO: update mutation

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

        toast.error(`Action failed. Reason: ${(result.error as ANY_TODO).data?.errors?.[fieldType]?.[0]}`);
      }
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Action failed. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Success! Your article has been published.");
    navigate(`${ROUTES.ARTICLE}/${result.data.article.slug}`);
  };

  return isLoading ? null : (
    <FormPage.Container title="Update Article">
      <EditorForm defaultValues={data?.article} onSubmit={onSubmit} disabled={isCreateArticleLoading} />
    </FormPage.Container>
  );
};

export type EditorFormProps = {
  onSubmit: (article: TEditorPageSchema) => void;
  disabled: boolean;
  defaultValues?: TEditorPageSchema;
};
export const EditorForm: FC<EditorFormProps> = ({ onSubmit, disabled, defaultValues }) => {
  const {
    control,
    setValue,
    watch,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(editorPageSchema),
    defaultValues: defaultValues ?? { ...editorPageDefaultValues },
  });

  return (
    <S.EditorForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={disabled}>
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
        <Button dataTestId="editor_submit_button" isDisabled={disabled}>
          Publish Article
        </Button>
      </S.SubmitButtonContainer>
    </S.EditorForm>
  );
};

export const EditorPage = () => null;
EditorPage.CreateView = CreateArcticle;
EditorPage.UpdateView = UpdateArticle;
