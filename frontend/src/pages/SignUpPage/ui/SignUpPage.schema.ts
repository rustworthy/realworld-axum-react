import * as z from "zod";

export const signUpPageSchema = z
  .object({
    username: z.string().nonempty({ error: "Cannot be empty." }),
    email: z.email({ error: "Valid email address required." }),
    password: z.string().nonempty({ error: "Cannot be empty." }),
    confirmPassword: z.string().nonempty({ error: "Cannot be empty." }),
    turnstileToken: z.string().nonempty({ error: "Cannot be empty." }),
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
  turnstileToken: "",
};

export type TSignUpPageSchema = z.infer<typeof signUpPageSchema>;
