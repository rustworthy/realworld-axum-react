import { FC } from "react";

import * as S from "./ActionButton.styles";
import { IButtonProps } from "./Button.types";

export const ActionButton: FC<IButtonProps> = (props) => {
  const { dataTestId, className = "SimpleButton", children, isDisabled, onClick } = props;

  return (
    <S.ActionButton disabled={isDisabled} onClick={onClick} data-testid={dataTestId} className={className}>
      {children}
    </S.ActionButton>
  );
};
