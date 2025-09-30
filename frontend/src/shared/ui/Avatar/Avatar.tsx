import { FC } from "react";

import * as S from "./Avatar.styles";

const FALLBACK_IMAGE_URL = "https://avatars.githubusercontent.com/u/4324516?v=4";

export type AvatarProps = {
  imageUrl?: string | null;
  username?: string;
  size?: "sm" | "md" | "lg";
};

export const Avatar: FC<AvatarProps> = ({ imageUrl, username, size = "sm" }) => {
  const altText = username ? `${username}'s profile picture` : "Avatar picture";
  return <S.AuthorImage $size={size} src={imageUrl ?? FALLBACK_IMAGE_URL} alt={altText} />;
};
