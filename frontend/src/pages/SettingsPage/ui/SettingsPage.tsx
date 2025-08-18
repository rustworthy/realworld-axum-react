import { Controller, useForm } from "react-hook-form";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useAuth } from "@/shared/auth";
import { FormPage } from "@/shared/ui/FormPage";
import { Button } from "@/shared/ui/controls/Button";
import { PasswordInput, TextInput, Textarea } from "@/shared/ui/controls/inputs";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TSettingsPageSchema, settingsPageDefaultValues, settingsPageSchema } from "./SettingsPage.schema";
import * as S from "./SettingsPage.styles";

export const SettingsPage = () => {
  const { update, isUpdateLoading, user, logout } = useAuth();

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(settingsPageSchema),
    defaultValues: { ...settingsPageDefaultValues, ...user, image: user!.image ?? "" },
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

    toast.success("Success! Your details have been successfully updated.");
  };

  return (
    <FormPage.Container title="Your Settings">
      <FormPage.Form noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isUpdateLoading}>
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
        <S.SubmitButtonContainer>
          <Button dataTestId="settings_submit_button" isDisabled={isUpdateLoading}>
            Update Settings
          </Button>
        </S.SubmitButtonContainer>
      </FormPage.Form>

      <S.LogoutSectionWrapper>
        <S.Separator />
        <S.LogoutButtonContainer>
          <Button dataTestId="settings_logout_button" onClick={logout}>
            Or click here to logout.
          </Button>
        </S.LogoutButtonContainer>
      </S.LogoutSectionWrapper>
    </FormPage.Container>
  );
};
