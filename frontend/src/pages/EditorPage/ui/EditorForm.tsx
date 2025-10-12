import { FC } from "react";
import { Controller, FieldErrors, useForm } from "react-hook-form";

import { formatCount } from "@/shared/lib/utils";
import { Button } from "@/shared/ui/controls/Button";
import { EditorInput, TextInput } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";

import { TEditorPageSchema, editorPageDefaultValues, editorPageSchema } from "./EditorPage.schema";
import * as S from "./EditorPage.styles";
import { FormPage } from "@/shared/ui/FormPage";

export type EditorFormProps = {
  maxLength: number;
  onSubmit: (article: TEditorPageSchema) => void;
  disabled: boolean;
  initialValues?: TEditorPageSchema;
  initialErrors?: FieldErrors<TEditorPageSchema>;
};
export const EditorForm: FC<EditorFormProps> = ({ onSubmit, disabled, initialValues, initialErrors, maxLength }) => {
  const {
    control,
    setValue,
    watch,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(editorPageSchema),
    defaultValues: initialValues ?? editorPageDefaultValues,
    errors: initialErrors,
  });

  return (
    <FormPage.Form noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={disabled}>
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
        label={`Write your article (in markdown, up to ${formatCount(maxLength)} characters)`}
        required
        id="editor_body"
        name="body"
        maxLength={maxLength}
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
    </FormPage.Form>
  );
};
