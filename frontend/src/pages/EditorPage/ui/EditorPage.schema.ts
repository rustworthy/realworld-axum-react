import * as z from "zod";

export const editorPageSchema = z.object({
  title: z.string().nonempty(),
  description: z.string().nonempty(),
  body: z.string().nonempty(),
  tagList: z.preprocess(csvTags => {
    if (typeof csvTags !== "string") return [];
    return csvTags.split(",").map(tag => tag.trim())
  },
    z.array(z.string().nonempty()).nonempty()),
});

export const editorPageDefaultValues: TEditorPageSchema = {
  title: "",
  description: "",
  body: "",
  tagList: [],
};

export type TEditorPageSchema = z.infer<typeof editorPageSchema>;
