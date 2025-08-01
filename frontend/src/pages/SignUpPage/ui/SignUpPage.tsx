import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useRegisterUserMutation } from "@/shared/api/generated";
import { SrOnlyLabel } from "@/shared/styles/globalStyledComponents";
import { AuthPage } from "@/shared/ui/AuthPage/AuthPage";
import { PasswordInput } from "@/shared/ui/AuthPage/PasswordInput";
import { Button } from "@/shared/ui/Button";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";
import * as z from "zod";

import * as S from "./SignUpPage.styles";

const schema = z.object({
  username: z.string().nonempty({ error: "Username cannot be empty." }),
  email: z.email({ error: "Valid email address required." }),
  password: z.string().nonempty({ error: "Password cannot be empty." }),
});

export const SignUpPage = () => {
  const navigate = useNavigate();
  const [registerUser, { isLoading }] = useRegisterUserMutation();
  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(schema),
    defaultValues: {
      username: "",
      email: "",
      password: "",
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
        <S.FormInputContainer>
          <SrOnlyLabel htmlFor="signup_username">Username</SrOnlyLabel>
          <S.FormInput
            {...register("username")}
            required
            id="signup_username"
            placeholder="Username"
            autoComplete="off"
            aria-invalid={!!errors.username}
            aria-errormessage="username_error"
          />
          {errors.username ? <S.FormInputError id="username_error">{errors.username.message}</S.FormInputError> : null}
        </S.FormInputContainer>

        <S.FormInputContainer>
          <SrOnlyLabel htmlFor="signup_email">Email</SrOnlyLabel>
          <S.FormInput
            {...register("email")}
            required
            id="signup_email"
            placeholder="Email"
            autoComplete="off"
            aria-invalid={!!errors.email}
            aria-errormessage="email_error"
          />
          {errors.email ? <S.FormInputError id="email_error">{errors.email.message}</S.FormInputError> : null}
        </S.FormInputContainer>

        <Controller
          control={control}
          name="password"
          render={({ field }) => (
            <PasswordInput
              {...field}
              required
              id="signup_password"
              label="Password"
              placeholder="Password"
              error={errors.password ? errors.password.message : undefined}
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
