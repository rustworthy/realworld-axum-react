import * as z from "zod";

export const settingsPageSchema = z.object({
  email: z.email({ error: "Valid email address required." }),

  username: z.string().nonempty({ error: "Cannot be empty." }),

  // leaving password blank, simply means not setting new password
  password: z.string().optional().nullable(),

  // setting bio will wipe the existing biography (if any)
  bio: z.string().optional().nullable(),

  // setting image URL to an empty string implies wiping it (if any existed)
  image: z.string().optional().nullable(),
});

export const settingsPageDefaultValues: TSettingsPageSchema = {
  image: "",
  username: "",
  bio: "",
  email: "",
  password: "",
};

export type TSettingsPageSchema = z.infer<typeof settingsPageSchema>;
