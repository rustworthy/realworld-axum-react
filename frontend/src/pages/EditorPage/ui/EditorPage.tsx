import { useState } from "react";
import { FieldErrors } from "react-hook-form";
import { useNavigate, useParams } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { NotFoundPage } from "@/pages/NotFoundPage";
import { useCreateArticleMutation, useReadArticleQuery, useUpdateArticleMutation } from "@/shared/api";
import { ROUTES } from "@/shared/constants/routes.constants";
import { ANY_TODO } from "@/shared/types/common.types";
import { FormPage } from "@/shared/ui/FormPage";
import { toast } from "sonner";

import { EditorForm } from "./EditorForm";
import { TEditorPageSchema } from "./EditorPage.schema";

const CreateArcticle = () => {
  const navigate = useNavigate();
  const [create, { isLoading }] = useCreateArticleMutation();
  const [initialErrors, setInitialErrors] = useState<FieldErrors<TEditorPageSchema> | undefined>(undefined);

  const onSubmit = async (data: TEditorPageSchema): Promise<void> => {
    const result = await create({
      articlePayloadArticleCreate: {
        article: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === 422) {
        const validationErrors: Record<keyof TEditorPageSchema, string[]> = (result.error as ANY_TODO).data.errors;
        const initialErrors: FieldErrors<TEditorPageSchema> = {};
        for (const [field, errors] of Object.entries(validationErrors)) {
          initialErrors[field as keyof TEditorPageSchema] = { type: "value", message: errors.join(". ") };
        }
        setInitialErrors(initialErrors);
        toast.error("Failed to publish the article. Please check field errors and re-submit.");
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
      <EditorForm initialErrors={initialErrors} onSubmit={onSubmit} disabled={isLoading} />
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
  const [update, { isLoading: isUpdateArticleLoading }] = useUpdateArticleMutation();
  const [initialErrors, setInitialErrors] = useState<FieldErrors<TEditorPageSchema> | undefined>(undefined);

  const onSubmit = async (data: TEditorPageSchema): Promise<void> => {
    const result = await update({
      slug: slug!,
      articlePayloadArticleUpdate: {
        article: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === 422) {
        const validationErrors: Record<keyof TEditorPageSchema, string[]> = (result.error as ANY_TODO).data.errors;
        const initialErrors: FieldErrors<TEditorPageSchema> = {};
        for (const [field, errors] of Object.entries(validationErrors)) {
          initialErrors[field as keyof TEditorPageSchema] = { type: "value", message: errors.join(". ") };
        }
        setInitialErrors(initialErrors);
        toast.error("Failed to update the article. Please check field errors and re-submit.");
      } else if ((result.error as FetchBaseQueryError).status === 404) {
        toast.error("Failed to update the article. Looks like article has been deleted.");
      } else if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Action failed. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Success! Your article has been published.");
    navigate(`${ROUTES.ARTICLE}/${result.data.article.slug}`);
  };

  return isLoading ? null : (
    <FormPage.Container title="Update Article">
      <EditorForm
        initialValues={data!.article}
        initialErrors={initialErrors}
        onSubmit={onSubmit}
        disabled={isUpdateArticleLoading}
      />
    </FormPage.Container>
  );
};

export const EditorPage = () => null;
EditorPage.CreateView = CreateArcticle;
EditorPage.UpdateView = UpdateArticle;
