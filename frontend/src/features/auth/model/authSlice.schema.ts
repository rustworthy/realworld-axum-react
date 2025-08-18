import z from "zod";

export const authSliceSchema = z.object({
  loadingCount: z.number(),
  isAuthenticated: z.boolean(),
  user: z
    .object({
      username: z.string().nonempty(),
      email: z.email(),
      image: z.string().nullable(),
      bio: z.string(),
      token: z.jwt(),
    })
    .nullable(),
});

export type AuthSliceState = z.infer<typeof authSliceSchema>;
