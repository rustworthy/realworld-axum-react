import * as z from "zod";

const nonempty = (fieldName: string) => {
  return z.preprocess(
    (v) => (typeof v === "string" ? v.trim() : v),
    z.string().nonempty({ message: `${fieldName} is required` }),
  );
};

export const editorPageSchema = z.object({
  title: nonempty("Title"),
  description: nonempty("Description"),
  body: nonempty("Article body"),
  tagList: z.preprocess(
    (csvTags) => {
      // the tags are already preprocessed, e.g. when
      // they are updating the existing articles; we are
      // not deduplicating here relying on us having done
      // this at the point when the article was created
      if (Array.isArray(csvTags)) return csvTags;
      // should not really happen, but we are handling
      // this case to "exhaust" the types
      if (typeof csvTags !== "string") return [];
      // they typed in comma separated tags, so we should
      // partse and deduplicate those
      const tags = csvTags
        .split(",")
        .map((tag) => tag.trim())
        .filter((tag) => tag !== "");
      return Array.from(new Set(tags));
    },
    z.array(z.string().nonempty({ message: "tag cannot be empty" })).nonempty({ message: "At least 1 tag is required" }),
  ),
});

export const editorPageDefaultValues: TEditorPageSchema = {
  title: "",
  description: "",
  body: "",
  tagList: [],
};

export type TEditorPageSchema = z.infer<typeof editorPageSchema>;
