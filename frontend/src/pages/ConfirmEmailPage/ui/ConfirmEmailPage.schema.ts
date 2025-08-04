import * as z from "zod";

const ALPHANUMERIC = /^[a-z0-9]+$/i;

export const OTP_LENGTH = 8;

export const confirmEmailSchema = z.object({
  otp: z
    .string()
    .length(OTP_LENGTH, { error: `Should contain ${OTP_LENGTH} characters.` })
    .refine((value) => ALPHANUMERIC.test(value), {
      error: "Should only contain letters and numbers.",
    }),
});

export const confirmEmailDefaultValues = {
  otp: "",
};

export type TConfirmEmail = z.infer<typeof confirmEmailSchema>;
