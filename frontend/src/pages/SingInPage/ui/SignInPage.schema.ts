import * as z from "zod";

export const signInPageSchema = z.object({
  email: z.email({ message: "Valid email address required." }),
  password: z.string().nonempty({ message: "Cannot be empty." }),
});

export const signInDefaultValues = {
  email: "",
  password: "",
};

export type TSignInPageSchema = z.infer<typeof signInPageSchema>;
