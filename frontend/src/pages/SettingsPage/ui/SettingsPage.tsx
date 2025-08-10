import { Controller, useForm } from "react-hook-form";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useAuth } from "@/shared/auth";
import { Button } from "@/shared/ui/controls/Button";
import { PasswordInput, TextInput, Textarea } from "@/shared/ui/controls/inputs";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TSettingsPageSchema, settingsPageDefaultValues, settingsPageSchema } from "./SettingsPage.schema";
import * as S from "./SettingsPage.styles";

export const SettingsPage = () => {
  const { update, isUpdateLoading, user } = useAuth();

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(settingsPageSchema),
    defaultValues: { ...settingsPageDefaultValues, ...user },
  });

  const onSubmit = async (data: TSettingsPageSchema): Promise<void> => {
    const result = await update({
      userPayloadUserUpdate: {
        user: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Failed to update settings. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Success! Your details have been succesfully updated.");
  };

  return (
    <>
      <AuthPageLayout title="Your Settings">
        <S.Form noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isUpdateLoading}>
          <Controller
            control={control}
            name="image"
            render={({ field }) => (
              <TextInput
                field={field}
                required
                id="settings_image"
                label="URL of profile picture"
                error={errors.image ? errors.image.message : undefined}
              />
            )}
          />

          <Controller
            control={control}
            name="username"
            render={({ field }) => (
              <TextInput
                field={field}
                required
                id="settings_username"
                label="Username"
                error={errors.username ? errors.username.message : undefined}
              />
            )}
          />

          <Controller
            control={control}
            name="bio"
            render={({ field }) => (
              <Textarea
                rows={8}
                field={field}
                required
                id="settings_bio"
                label="Short bio about you"
                error={errors.bio ? errors.bio.message : undefined}
              />
            )}
          />

          <Controller
            control={control}
            name="email"
            render={({ field }) => (
              <TextInput
                field={field}
                required
                id="signup_email"
                label="Email"
                error={errors.email ? errors.email.message : undefined}
              />
            )}
          />

          <Controller
            control={control}
            name="password"
            render={({ field }) => (
              <PasswordInput
                field={field}
                required
                id="settings_password"
                label="New Password"
                error={errors.password ? errors.password.message : undefined}
              />
            )}
          />
          <S.ButtonContainer>
            <Button dataTestId="settings_submit_button" isDisabled={isUpdateLoading}>
              Update Settings
            </Button>
          </S.ButtonContainer>
        </S.Form>
        <S.Separator />

        <S.LogoutButtonContainer>
          <Button dataTestId="settings_logout_button">
            Or click here to logout.
          </Button>
        </S.LogoutButtonContainer>
      </AuthPageLayout>
      <div className="settings-page">
        <div className="container page">
          <div className="row">
            <div className="col-md-6 offset-md-3 col-xs-12">
              <h1 className="text-xs-center">Your Settings</h1>

              <ul className="error-messages">
                <li>That name is required</li>
              </ul>

              <form>
                <fieldset>
                  <fieldset className="form-group">
                    <input className="form-control" type="text" placeholder="URL of profile picture" />
                  </fieldset>
                  <fieldset className="form-group">
                    <input className="form-control form-control-lg" type="text" placeholder="Your Name" />
                  </fieldset>
                  <fieldset className="form-group">
                    <textarea className="form-control form-control-lg" rows={8} placeholder="Short bio about you" />
                  </fieldset>
                  <fieldset className="form-group">
                    <input className="form-control form-control-lg" type="text" placeholder="Email" />
                  </fieldset>
                  <fieldset className="form-group">
                    <input className="form-control form-control-lg" type="password" placeholder="New Password" />
                  </fieldset>
                  <button className="btn btn-lg btn-primary pull-xs-right">Update Settings</button>
                </fieldset>
              </form>
              <hr />
              <button className="btn btn-outline-danger">Or click here to logout.</button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};
