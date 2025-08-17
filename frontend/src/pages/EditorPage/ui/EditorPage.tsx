import { Controller, useForm } from "react-hook-form";

import { FormPage } from "@/shared/ui/FormPage";
import { Button } from "@/shared/ui/controls/Button";
import { PasswordInput, TextInput, Textarea } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";

import { TEditorPageSchema, editorPageDefaultValues, editorPageSchema } from "./EditorPage.schema";
import * as S from "./EditorPage.styles";

export const EditorPage = () => {
  const isPublishLoading = false;

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(editorPageSchema),
    defaultValues: { ...editorPageDefaultValues },
  });

  const onSubmit = async (data: TEditorPageSchema): Promise<void> => {
    console["log"](data);
  };

  return (
    <>
      <FormPage.Container title="Your Settings">
        <S.EditorForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isPublishLoading}>
          <Controller
            control={control}
            name="title"
            render={({ field }) => (
              <TextInput
                field={field}
                required
                id="editor_title"
                label="Article's title"
                error={errors.title?.message}
              />
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

          <Controller
            control={control}
            name="body"
            render={({ field }) => (
              <Textarea
                rows={8}
                field={field}
                required
                id="editor_body"
                label="Write your article (in markdown)"
                error={errors.body?.message}
              />
            )}
          />

          <Controller
            control={control}
            name="tagList"
            render={({ field }) => (
              <TextInput
                field={field}
                required
                id="editor_tags"
                label="Enter tags"
                error={errors.tagList?.message}
              />
            )}
          />

          <S.SubmitButtonContainer>
            <Button dataTestId="editor_submit_button" isDisabled={isPublishLoading}>
              Publish Article
            </Button>
          </S.SubmitButtonContainer>
        </S.EditorForm>
      </FormPage.Container>

      <div className="editor-page">
        <div className="container page">
          <div className="row">
            <div className="col-md-10 offset-md-1 col-xs-12">
              <form>
                <fieldset>
                  <fieldset className="form-group">
                    <input type="text" className="form-control form-control-lg" placeholder="Article Title" />
                  </fieldset>
                  <fieldset className="form-group">
                    <input type="text" className="form-control" placeholder="What's this article about?" />
                  </fieldset>
                  <fieldset className="form-group">
                    <textarea
                      className="form-control"
                      rows={8}
                      placeholder="Write your article (in markdown)"
                    ></textarea>
                  </fieldset>
                  <fieldset className="form-group">
                    <input type="text" className="form-control" placeholder="Enter tags" />
                    <div className="tag-list">
                      <span className="tag-default tag-pill"> <i className="ion-close-round"></i> tag </span>
                    </div>
                  </fieldset>
                  <button className="btn btn-lg pull-xs-right btn-primary" type="button">
                    Publish Article
                  </button>
                </fieldset>
              </form>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};
