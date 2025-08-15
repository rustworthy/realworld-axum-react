import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useRegisterUserMutation } from "@/shared/api/generated";
import { ROUTES } from "@/shared/constants/routes.constants";
import { Button } from "@/shared/ui/controls/Button";
import { PasswordInput, TextInput } from "@/shared/ui/controls/inputs";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TSignUpPageSchema, signUpDefaultValues, signUpPageSchema } from "./SignUpPage.schema";
import * as S from "./SignUpPage.styles";
import { Turnstile } from "@marsidev/react-turnstile";
import { useState } from "react";
import { useTernaryDarkMode } from "usehooks-ts";

export const SignUpPage = () => {
  const { ternaryDarkMode } = useTernaryDarkMode();
  const turnstileMode = ternaryDarkMode === "system" ? "auto" : ternaryDarkMode === "dark" ? "dark" : "light";
  const [token, setToken] = useState<string | null>(null);
  const navigate = useNavigate();
  const [registerUser, { isLoading }] = useRegisterUserMutation();

  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(signUpPageSchema),
    defaultValues: signUpDefaultValues,
  });

  const onSubmit = async (data: TSignUpPageSchema): Promise<void> => {
    console.log(token);
    return;
    const result = await registerUser({
      userPayloadRegistration: {
        user: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Failed to register. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Great! Let's confirm your email address. Please check your inbox.");
    navigate(ROUTES.CONFIRM_EMAIL);
  };

  return (
    <AuthPageLayout title="Sign up">
      <S.SignInLink href="/signin">Have an account?</S.SignInLink>

      <S.SignUpForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isLoading}>
        <Controller
          control={control}
          name="username"
          render={({ field }) => (
            <TextInput
              field={field}
              required
              id="signup_username"
              label="Username"
              error={errors.username ? errors.username.message : undefined}
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
              autoComplete="email"
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
              id="signup_password"
              label="Password"
              error={errors.password ? errors.password.message : undefined}
            />
          )}
        />

        <Controller
          control={control}
          name="confirmPassword"
          render={({ field }) => (
            <PasswordInput
              field={field}
              required
              id="signup_password_confirm"
              label="Confirm Password"
              error={errors.confirmPassword ? errors.confirmPassword.message : undefined}
            />
          )}
        />

        <S.SignUpButtonContainer>
          <div style={{ width: "300px", height: "65px" }}>
            <Turnstile siteKey="1x00000000000000000000AA" onSuccess={setToken} options={{ theme: turnstileMode }} />
          </div>
          <Button dataTestId="signup_submit_button" isDisabled={isLoading}>
            Sign up
          </Button>
        </S.SignUpButtonContainer>
      </S.SignUpForm>
    </AuthPageLayout>
  );
};
