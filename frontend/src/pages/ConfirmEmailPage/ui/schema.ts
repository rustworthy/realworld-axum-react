import * as z from "zod";

export const confirmEmailSchema = z.object({
  otp: z.string().length(8, { error: "Please type in full OTP" }),
});

export const confirmEmailDefaultValues = {
  otp: "",
};

export type TConfirmEmail = z.infer<typeof confirmEmailSchema>;
