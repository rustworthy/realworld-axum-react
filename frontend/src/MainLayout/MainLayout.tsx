import { FC, ReactNode } from "react";
import * as S from "./MainLayout.styles";

export interface IMainLayout {
  children: ReactNode;
}

export const MainLayout: FC<IMainLayout> = ({ children }) => {
  return (
    <S.Wrapper>
      <S.Header>
        <S.HeaderContainer>
          <S.HeaderLink href="#">conduit</S.HeaderLink>

          <nav>
            <S.HeaderNavList>
              <li>Home</li>
              <li>Sing in</li>
              <li>Sing up</li>
            </S.HeaderNavList>
          </nav>
        </S.HeaderContainer>
      </S.Header>

      <S.ChildrenContainer>
        <S.Container>{children}</S.Container>
      </S.ChildrenContainer>

      <S.Footer>
        <S.FooterContainer>
          <S.FooterLink href="https://github.com/rustworthy/realworld-rocket-react" target="_blank">Fork on GitHub</S.FooterLink>
        </S.FooterContainer>
      </S.Footer>
    </S.Wrapper>
  );
};
