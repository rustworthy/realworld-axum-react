import { PASSWORD_MIN_LENGTH } from "@/shared/constants/auth.constants";
import * as z from "zod";

export const signUpPageSchema = z
  .object({
    username: z.string().nonempty({ error: "Cannot be empty." }),
    email: z.email({ error: "Valid email address required." }),
    password: z.string().min(PASSWORD_MIN_LENGTH, `Password should be at least ${PASSWORD_MIN_LENGTH} characters long.`),
    confirmPassword: z.string().nonempty({ error: "Cannot be empty." }),
    captcha: z.string().nonempty({ error: "Cannot be empty." }),
  })
  .refine((data) => data.confirmPassword === data.password, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

export const signUpDefaultValues = {
  username: "",
  email: "",
  password: "",
  confirmPassword: "",
  captcha: "",
};

export type TSignUpPageSchema = z.infer<typeof signUpPageSchema>;
