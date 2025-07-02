import { Suspense } from "react";
import * as S from "./MainLayout.styles";
import { Outlet, useNavigate } from "react-router-dom";


export const MainLayout = () => {
  const navigate = useNavigate();

  return (
    <S.Wrapper>
      <S.Header>
        <S.HeaderContainer>
          <S.HeaderLink href="#">conduit</S.HeaderLink>

          <nav>
            <S.HeaderNavList>
              <li onClick={() => navigate("/")}>Home</li>
              <li onClick={() => navigate("/singin")}>Sing in</li>
              <li onClick={() => navigate("/singup")}>Sing up</li>
            </S.HeaderNavList>
          </nav>
        </S.HeaderContainer>
      </S.Header>

      <S.ChildrenContainer>
          <Suspense fallback={<div>Suspense loader will be here</div>}>
            <Outlet />
          </Suspense>
      </S.ChildrenContainer>

      <S.Footer>
        <S.FooterContainer>
          <S.FooterLink href="https://github.com/rustworthy/realworld-rocket-react" target="_blank">
            Fork on GitHub
          </S.FooterLink>
        </S.FooterContainer>
      </S.Footer>
    </S.Wrapper>
  );
};
