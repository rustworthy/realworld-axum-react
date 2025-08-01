import * as z from "zod";

export const registrationSchema = z
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

export type Registration = z.infer<typeof registrationSchema>;
