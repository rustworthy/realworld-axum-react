import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useAuth } from "@/shared/auth";
import { ROUTES } from "@/shared/constants/routes.constants";
import { Button } from "@/shared/ui/controls/Button";
import { PasswordInput, TextInput, TurnstileInput } from "@/shared/ui/controls/inputs";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { TSignInPageSchema, signInDefaultValues, signInPageSchema } from "./SignInPage.schema";
import * as S from "./SignInPage.styles";

export const SignInPage = () => {
  const navigate = useNavigate();
  const { login, isLoginLoading } = useAuth();

  const {
    control,
    handleSubmit,
    setValue, setError, clearErrors,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(signInPageSchema),
    defaultValues: signInDefaultValues,
  });

  const onSubmit = async (data: TSignInPageSchema): Promise<void> => {
    const result = await login({
      userPayloadLogin: {
        user: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Failed to sign in. Please check your internet connection and retry.");
      } else {
        toast.error("Invalid email or password.");
      }
      return;
    }

    toast.success("Welcome back!");
    navigate(ROUTES.HOME);
  };

  return (
    <AuthPageLayout title="Sign in">
      <S.SignUpLink href="/signup">Need an account?</S.SignUpLink>

      <S.SignInForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isLoginLoading}>
        <Controller
          control={control}
          name="email"
          render={({ field }) => (
            <TextInput
              field={field}
              required
              id="signin_email"
              label="Email"
              autoComplete="email"
              error={errors.email?.message}
            />
          )}
        />

        <Controller
          control={control}
          name="password"
          render={({ field }) => (
            <PasswordInput field={field} required id="signin_password" label="Password" error={errors.password?.message} />
          )}
        />

        <S.SubmissionSection>
          <TurnstileInput
            name="turnstileToken"
            setValue={setValue}
            setError={setError}
            fieldErrors={errors}
            clearErrors={clearErrors}
          />
          <Button dataTestId="signin_submit_button" isDisabled={isLoginLoading}>
            Sign in
          </Button>
        </S.SubmissionSection>
      </S.SignInForm>
    </AuthPageLayout>
  );
};
