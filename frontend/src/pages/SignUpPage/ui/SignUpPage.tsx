import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useRegisterUserMutation } from "@/shared/api/generated";
import { AuthPage } from "@/shared/ui/AuthPage/AuthPage";
import { PasswordInput } from "@/shared/ui/AuthPage/PasswordInput";
import { TextInput } from "@/shared/ui/AuthPage/TextInput";
import { Button } from "@/shared/ui/Button";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import * as z from "zod";

import * as S from "./SignUpPage.styles";

const schema = z
  .object({
    username: z.string().nonempty({ error: "Cannot be empty." }),
    email: z.email({ error: "Valid email address required." }),
    password: z.string().nonempty({ error: "Cannot be empty." }),
    confirmPassword: z.string().nonempty({ error: "Cannot be empty." }),
  })
  .refine((data) => data.confirmPassword === data.password, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

export const SignUpPage = () => {
  const navigate = useNavigate();
  const [registerUser, { isLoading }] = useRegisterUserMutation();
  const {
    control,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(schema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
  });

  const onSubmit = async (data: z.infer<typeof schema>) => {
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
    navigate("/confirm-email");
  };

  return (
    <AuthPage title="Sign up">
      <S.SignInLink href="/signin">Have an account?</S.SignInLink>

      <S.SignUpForm noValidate onSubmit={handleSubmit(onSubmit)}>
        <Controller
          control={control}
          name="username"
          render={({ field }) => (
            <TextInput
              {...field}
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
              {...field}
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
              {...field}
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
              {...field}
              required
              id="signup_password_confirm"
              label="Confirm Password"
              error={errors.confirmPassword ? errors.confirmPassword.message : undefined}
            />
          )}
        />

        <S.SignUpButtonContainer>
          <Button dataTestId="signup_submit_button" isDisabled={isLoading}>
            Sign up
          </Button>
        </S.SignUpButtonContainer>
      </S.SignUpForm>
    </AuthPage>
  );
};
