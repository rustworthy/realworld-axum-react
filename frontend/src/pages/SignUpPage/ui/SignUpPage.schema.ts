import * as z from "zod";

// https://stackoverflow.com/a/21456918
const PASSWORD_REGEX = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{12,}$/;

export const signUpPageSchema = z
  .object({
    username: z.string().nonempty({ error: "Cannot be empty." }),
    email: z.email({ error: "Valid email address required." }),
    password: z
      .string()
      .nonempty({ error: "Cannot be empty." })
      .refine((value) => PASSWORD_REGEX.test(value), {
        error:
          "Password should be at least 12 characters long and include at least 1 uppercase letter, 1 lowercase letter, 1 digit, and a special character ($, !, %, *, ?, &)",
      }),
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
