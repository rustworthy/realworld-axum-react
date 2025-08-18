import { FC } from "react";

import * as S from "./Loader.styles";

export const Loader: FC = () => {
  return (
    <S.LoaderBackdrop>
      <S.LoaderContainer role="progressbar" aria-label="loading" />
    </S.LoaderBackdrop>
  );
};
