import z from "zod";

export const loadingSliceSchema = z.object({
  loadingCount: z.number(),
});

export type LoadingSliceState = z.infer<typeof loadingSliceSchema>;
