import { PASSWORD_MIN_LENGTH } from "@/shared/constants/auth.constants";
import * as z from "zod";

export const signInPageSchema = z.object({
  email: z.email({ message: "Valid email address required." }),
  password: z.string().min(PASSWORD_MIN_LENGTH, `Password should be at least ${PASSWORD_MIN_LENGTH} characters long.`),
  captchaToken: z.string().nonempty({ error: "Cannot be empty." }),
});

export const signInDefaultValues = {
  email: "",
  password: "",
  captchaToken: "",
};

export type TSignInPageSchema = z.infer<typeof signInPageSchema>;
