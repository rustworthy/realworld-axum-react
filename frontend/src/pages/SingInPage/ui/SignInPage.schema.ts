import * as z from "zod";

export const signInPageSchema = z.object({
  email: z.email({ message: "Valid email address required." }),
  password: z.string().nonempty({ message: "Cannot be empty." }),
  turnstileToken: z.string().nonempty({ error: "Cannot be empty." }),
});

export const signInDefaultValues = {
  email: "",
  password: "",
  turnstileToken: "",
};

export type TSignInPageSchema = z.infer<typeof signInPageSchema>;
