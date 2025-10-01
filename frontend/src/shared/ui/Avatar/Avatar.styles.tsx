import styled from "@emotion/styled";

export const AuthorImage = styled.img<{ $size: "sm" | "md" | "lg" }>`
  width: ${({ $size: size }) => (size === "sm" ? "2rem" : size === "md" ? "4rem" : "6rem")};
  height: ${({ $size: size }) => (size === "sm" ? "2rem" : size === "md" ? "4rem" : "6rem")};
  object-fit: cover;
  object-position: top center;
  border-radius: 50%;
`;
