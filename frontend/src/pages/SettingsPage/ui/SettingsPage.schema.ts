import { PASSWORD_MIN_LENGTH } from "@/shared/constants/auth.constants";
import * as z from "zod";

export const settingsPageSchema = z.object({
  email: z.email({ error: "Valid email address required." }),
  username: z.string().nonempty({ error: "Cannot be empty." }),
  password: z.preprocess(
    (value) => (value === "" ? undefined : value),
    z.string().min(PASSWORD_MIN_LENGTH, `Password should be at least ${PASSWORD_MIN_LENGTH} characters long.`).optional(),
  ),
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
