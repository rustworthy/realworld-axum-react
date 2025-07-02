import { FC } from "react";

import * as S from "./Button.styles";
import { IButton } from "./Button.types";

export const Button: FC<IButton> = (props) => {
  const { dataTestId, children, isDisabled, onClick } = props;

  return (
    <S.Button disabled={isDisabled} onClick={onClick} data-testid={dataTestId}>
      {children}
    </S.Button>
  );
};
