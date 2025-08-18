/**
 * Centralized place for application configuration, including things that
 * we what the bundler to pull from environment.
 *
 * NB. Make sure environment variables from below are listed in `EnvironmentPlugin`
 * in `frontend/configs/webpack/buildPlugins.ts
 */
export const config = {
  BACKEND_URL: process.env.BACKEND_URL!,
  TURNSTILE_SITE_KEY: process.env.TURNSTILE_SITE_KEY!,
  REACT_APP_MODE: process.env.REACT_APP_MODE!,
} as const;
