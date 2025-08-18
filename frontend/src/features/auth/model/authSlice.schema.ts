import z from "zod";

export const authSliceSchema = z.object({
  isAuthenticated: z.boolean(),
  user: z
    .object({
      username: z.string().nonempty(),
      email: z.email(),
      image: z.url().nullable(),
      bio: z.string(),
      token: z.jwt(),
    })
    .nullable(),
});

export type AuthSliceState = z.infer<typeof authSliceSchema>;
