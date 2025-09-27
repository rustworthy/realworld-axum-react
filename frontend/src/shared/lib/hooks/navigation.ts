import { useEffect, useRef } from "react";
import { useLocation } from "react-router";

export function useHashScrollIn(awaitedResource?: unknown) {
  const scrolled = useRef(false);
  const location = useLocation();

  useEffect(() => {
    // only scroll if that's a first load with resource already loaded/fetched:
    // we do not want to scroll again when they mutate the data, e.g. favorite
    // or unfavorite this article;
    if (scrolled.current) return;
    const id = location.hash.slice(1);
    if (id) {
      const element = document.getElementById(id);
      if (element) {
        scrolled.current = true;
        setTimeout(() => element.scrollIntoView({ behavior: "smooth", block: "start" }), 500);
      }
    }
    // initally, the data (e.g. article) may not be in the cache and
    // so we want to check again once the resource is fully loaded (e.g. the
    // artcile's body is there and may contain the hash we are looking for)
  }, [awaitedResource, location.hash]);
}
