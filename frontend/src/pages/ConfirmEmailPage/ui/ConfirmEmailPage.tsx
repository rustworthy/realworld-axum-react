import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router";

import { Button } from "@/shared/ui/controls/Button";
import { OTPInput } from "@/shared/ui/controls/inputs/OTPInput";
import { AuthPageLayout } from "@/shared/ui/layouts";
import { zodResolver } from "@hookform/resolvers/zod";

import * as S from "./ConfirmEmailPage.styles";
import { OTP_LENGTH, TConfirmEmail, confirmEmailDefaultValues, confirmEmailSchema } from "./schema";

export const ConfirmEmailPage = () => {
  const _navigate = useNavigate();
  const onSubmit = async (data: TConfirmEmail) => {
    // eslint-disable-next-line no-console
    console.log(data);
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
      <S.OTPForm noValidate onSubmit={handleSubmit(onSubmit)}>
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
          <Button dataTestId="confirm_email_button" isDisabled={false}>
            Submit
          </Button>
        </S.ButtonContainer>
      </S.OTPForm>
    </AuthPageLayout>
  );
};
