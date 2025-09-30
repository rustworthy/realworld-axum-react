/* eslint-disable react-refresh/only-export-components */
import { FC, PropsWithChildren } from "react";

import * as S from "./Tabs.styles";

const List: FC<PropsWithChildren> = ({ children }) => {
  return (
    <S.TabContainer>
      <S.TabList>{children}</S.TabList>
    </S.TabContainer>
  );
};

const Item: FC<{ label: string; to: string; isActive: boolean }> = ({ label, to, isActive }) => (
  <S.TabItem>
    <S.TabLink $isActive={isActive} to={to}>
      {label}
    </S.TabLink>
  </S.TabItem>
);

export const Tabs = {
  List,
  Item,
};
