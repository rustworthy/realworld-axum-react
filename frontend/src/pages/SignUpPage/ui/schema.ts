import * as z from "zod";

export const signupSchema = z
  .object({
    username: z.string().nonempty({ error: "Cannot be empty." }),
    email: z.email({ error: "Valid email address required." }),
    password: z.string().nonempty({ error: "Cannot be empty." }),
    confirmPassword: z.string().nonempty({ error: "Cannot be empty." }),
  })
  .refine((data) => data.confirmPassword === data.password, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

export const signupDefaultValues = {
  username: "",
  email: "",
  password: "",
  confirmPassword: "",
};

export type TSignup = z.infer<typeof signupSchema>;
