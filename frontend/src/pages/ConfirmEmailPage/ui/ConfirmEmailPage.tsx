import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useAuth } from "@/shared/auth";
import { Button } from "@/shared/ui/controls/Button";
import { OTPInput } from "@/shared/ui/controls/inputs";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { OTP_LENGTH, TConfirmEmail, confirmEmailDefaultValues, confirmEmailSchema } from "./ConfirmEmailPage.schema";
import * as S from "./ConfirmEmailPage.styles";

export const ConfirmEmailPage = () => {
  const navigate = useNavigate();
  const { confirmEmail, isConfirmEmailLoading } = useAuth();

  const onSubmit = async (data: TConfirmEmail) => {
    const result = await confirmEmail({
      userPayloadEmailConfirmation: {
        user: data,
      },
    });

    if (result.error) {
      if ((result.error as FetchBaseQueryError).status === "FETCH_ERROR") {
        toast.error("Failed to confirm the email address. Please check your internet connection and retry.");
      }
      return;
    }

    toast.success("Welcome! Let's update your profile.");
    navigate("/settings");
  };

  const {
    handleSubmit,
    control,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(confirmEmailSchema),
    defaultValues: confirmEmailDefaultValues,
  });

  return (
    <AuthPageLayout title="Let's confirm your email">
      <S.OTPInstruction>Please insert a one-time code we've sent to you via email.</S.OTPInstruction>
      <S.OTPForm noValidate onSubmit={handleSubmit(onSubmit)} aria-disabled={isConfirmEmailLoading}>
        <Controller
          control={control}
          name="otp"
          render={({ field }) => (
            <OTPInput
              {...field}
              length={OTP_LENGTH}
              required
              label="One time code for email confirmation"
              id="confirm_email_otp"
              error={errors.otp ? errors.otp.message : undefined}
            />
          )}
        />

        <S.ButtonContainer>
          <Button dataTestId="confirm_email_button" isDisabled={isConfirmEmailLoading}>
            Submit
          </Button>
        </S.ButtonContainer>
      </S.OTPForm>
    </AuthPageLayout>
  );
};
