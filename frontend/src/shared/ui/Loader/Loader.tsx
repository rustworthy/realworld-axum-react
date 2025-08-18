import { FC } from "react";

import * as S from "./Loader.styles";

// Loader Component from https://cssloaders.github.io/
export const Loader: FC = () => {
  return (
    <S.LoaderBackdrop>
      <S.LoaderContainer role="progressbar" aria-label="loading" />
    </S.LoaderBackdrop>
  );
};
