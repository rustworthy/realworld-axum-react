import { Controller, useForm } from "react-hook-form";
import { useNavigate, useSearchParams } from "react-router";

import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { useAuth } from "@/shared/auth";
import { ROUTES } from "@/shared/constants/routes.constants";
import { Button } from "@/shared/ui/controls/Button";
import { OTPInput, CaptchaInput } from "@/shared/ui/controls/inputs";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";
import { toast } from "sonner";

import { OTP_LENGTH, TConfirmEmail, confirmEmailDefaultValues, confirmEmailSchema } from "./ConfirmEmailPage.schema";
import * as S from "./ConfirmEmailPage.styles";

export const ConfirmEmailPage = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const initialOTP = (searchParams.get("otp") ?? "").slice(0, OTP_LENGTH);
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
    navigate(ROUTES.SETTINGS);
  };

  const {
    handleSubmit,
    setValue, setError, clearErrors,
    control,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(confirmEmailSchema),
    defaultValues: { ...confirmEmailDefaultValues, otp: initialOTP },
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

        <S.SubmissionSection>
          <CaptchaInput
            name="captchaToken"
            setValue={setValue}
            setError={setError}
            fieldErrors={errors}
            clearErrors={clearErrors}
          />
          <Button dataTestId="confirm_email_button" isDisabled={isConfirmEmailLoading}>
            Submit
          </Button>
        </S.SubmissionSection>
      </S.OTPForm>
    </AuthPageLayout>
  );
};
