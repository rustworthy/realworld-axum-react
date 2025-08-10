import * as z from "zod";

export const settingsPageSchema = z.object({
  email: z.email({ error: "Valid email address required." }),
  username: z.string().nonempty({ error: "Cannot be empty." }),
  password: z.string(),
  bio: z.string(),
  image: z.preprocess((val) => (val === "" ? null : val), z.url({ message: "Valid URL required." }).nullable()),
});

export const settingsPageDefaultValues: TSettingsPageSchema = {
  email: "",
  username: "",
  password: "",
  bio: "",
  image: "",
};

export type TSettingsPageSchema = z.infer<typeof settingsPageSchema>;
