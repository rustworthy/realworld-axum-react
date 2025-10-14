import * as z from "zod";

const NUMERIC = /^[0-9]+$/i;

export const OTP_LENGTH = 8;

export const confirmEmailSchema = z.object({
  otp: z
    .string()
    .length(OTP_LENGTH, { error: "Invalid OTP length." })
    .refine((value) => NUMERIC.test(value), {
      error: "Should only contain numbers.",
    }),
  captcha: z.string().nonempty({ error: "Cannot be empty." }),
});

export const confirmEmailDefaultValues: TConfirmEmail = {
  otp: "",
  captcha: "",
};

export type TConfirmEmail = z.infer<typeof confirmEmailSchema>;
