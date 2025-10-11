import * as z from "zod";

export const COMMENT_MAX_LENGTH = 500;

export const createCommentSchema = z.object({
  body: z
    .string()
    .nonempty({ error: "Cannot be empty." })
    .max(COMMENT_MAX_LENGTH, { message: `Comment cannot exceed ${COMMENT_MAX_LENGTH} characters.` }),
});

export type TCreateCommentSchema = z.infer<typeof createCommentSchema>;

export const createCommentDefaultValues: TCreateCommentSchema = {
  body: "",
};
