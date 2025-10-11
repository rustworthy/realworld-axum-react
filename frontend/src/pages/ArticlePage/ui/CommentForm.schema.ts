import * as z from "zod";

export const createCommentSchema = z.object({
  body: z.string().nonempty({ error: "Cannot be empty." }).max(500),
});

export type TCreateCommentSchema = z.infer<typeof createCommentSchema>;

export const createCommentDefaultValues: TCreateCommentSchema = {
  body: "",
};
