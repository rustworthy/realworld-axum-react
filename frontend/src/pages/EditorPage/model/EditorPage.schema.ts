import * as z from "zod";

export const ARTICLE_MAX_LENGTH = 12_500;

const nonempty = (fieldName: string, maxLength: number) => {
  return z.preprocess(
    (v) => (typeof v === "string" ? v.trim() : v),
    z
      .string()
      .nonempty({ message: `${fieldName} is required` })
      .max(maxLength),
  );
};

export const editorPageSchema = z.object({
  title: nonempty("Title", 200),
  description: nonempty("Description", 200),
  body: nonempty("Article body", ARTICLE_MAX_LENGTH),
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
    z
      .array(z.string().nonempty({ message: "tag cannot be empty" }).max(15))
      .nonempty({ message: "At least 1 tag is required" })
      .max(10),
  ),
});

export const editorPageDefaultValues: TEditorPageSchema = {
  title: "",
  description: "",
  body: "",
  tagList: [],
};

export type TEditorPageSchema = z.infer<typeof editorPageSchema>;
