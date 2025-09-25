import { SerializedError } from "@reduxjs/toolkit";
import { FetchBaseQueryError } from "@reduxjs/toolkit/query";

import { ANY_TODO } from "@/shared/types/common.types";

// TODO: this will naively parse out the first error message in case
// of the unprocessable entity error returned from the server; this only
// serves as a temporary solution. Most of the time we are working with
// forms and so should parse the returned error in details and render those
// message per form field;
export function parseOutErrorMessage(error: FetchBaseQueryError | SerializedError): string {
  if ((error as FetchBaseQueryError).status === 422) {
    const errors = (error as ANY_TODO).data?.errors;
    const fieldType = Object.keys(errors)[0];
    return `Action failed. Reason: ${errors?.[fieldType]?.[0]}`;
  }
  if ((error as FetchBaseQueryError).status === "FETCH_ERROR") {
    return "Action failed. Please check your internet connection and retry.";
  }
  return "Unexpected error";
}

export function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

export function formatUsername(username: string, maxLen = 15): string {
  return username.length > maxLen ? `${username.slice(0, maxLen - 3)}...` : username;
}

/**
 * Format events (likes, views, etc.) string.
 *
 * Values exceeding 1000 will be formatted using float plus `k`,
 * e.g. 1344 likes will formatted as `1.3k`.
 */
export function formatEventsCount(count: number): string {
  return count < 1_000 ? count.toString() : `${(count / 1_000).toFixed(1)}k`;
}
