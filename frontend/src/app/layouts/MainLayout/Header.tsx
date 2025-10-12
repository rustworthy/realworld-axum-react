import { useLocation, useNavigate } from "react-router";

import { useAuth } from "@/features/auth";
import { ROUTES } from "@/shared/constants/routes.constants";
import { truncateText } from "@/shared/lib/utils";
import { Avatar } from "@/shared/ui/Avatar";
import * as DropdownMenu from "@radix-ui/react-dropdown-menu";
import { GearIcon, HamburgerMenuIcon, HomeIcon, Pencil2Icon } from "@radix-ui/react-icons";

import * as S from "./Header.styles";

export const Header = () => {
  const navigate = useNavigate();
  const { pathname } = useLocation();
  const { isAuthenticated, user } = useAuth();

  return (
    <header>
      <S.HeaderContainer>
        <S.HeaderLink to={ROUTES.HOME}>conduit</S.HeaderLink>
        {/* ----------------------- mobiles and tablets --------------------- */}
        <S.DropDownMenuWrapper>
          <DropdownMenu.Root>
            <DropdownMenu.Trigger asChild>
              <button className="IconButton" aria-label="Customise options">
                <HamburgerMenuIcon />
              </button>
            </DropdownMenu.Trigger>
            <DropdownMenu.Portal>
              <DropdownMenu.Content style={{ background: "white" }} sideOffset={5}>
                {isAuthenticated ? (
                  <>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                        <HomeIcon />
                        Home
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem $isActive={pathname === ROUTES.EDITOR} onClick={() => navigate(ROUTES.EDITOR)}>
                        <Pencil2Icon />
                        New Article
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem
                        className="Compact"
                        $isActive={pathname === ROUTES.SETTINGS}
                        onClick={() => navigate(ROUTES.SETTINGS)}
                      >
                        <GearIcon />
                        Settings
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem
                        $isActive={pathname === `${ROUTES.PROFILE}/${user!.username}`}
                        onClick={() => navigate(`${ROUTES.PROFILE}/${user!.username}`)}
                      >
                        <Avatar imageUrl={user!.image} username={user!.username} />
                        {truncateText(user!.username, 12)}
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                  </>
                ) : (
                  <>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                        <HomeIcon />
                        Home
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>{" "}
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNIN} onClick={() => navigate(ROUTES.SIGNIN)}>
                        Sign in
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                    <DropdownMenu.Item className="">
                      <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNUP} onClick={() => navigate(ROUTES.SIGNUP)}>
                        Sign up
                      </S.HeaderNavItem>
                    </DropdownMenu.Item>
                  </>
                )}
              </DropdownMenu.Content>
            </DropdownMenu.Portal>
          </DropdownMenu.Root>
        </S.DropDownMenuWrapper>
        {/* --------------------------- larger screens --------------------- */}
        <S.NavWrapper>
          <S.HeaderNavList>
            {isAuthenticated ? (
              <>
                <li>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                    <HomeIcon />
                    Home
                  </S.HeaderNavItem>
                </li>
                <li>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.EDITOR} onClick={() => navigate(ROUTES.EDITOR)}>
                    <Pencil2Icon />
                    New Article
                  </S.HeaderNavItem>
                </li>
                <li>
                  <S.HeaderNavItem
                    className="Compact"
                    $isActive={pathname === ROUTES.SETTINGS}
                    onClick={() => navigate(ROUTES.SETTINGS)}
                  >
                    <GearIcon />
                    Settings
                  </S.HeaderNavItem>
                </li>
                <li>
                  <S.HeaderNavItem
                    $isActive={pathname === `${ROUTES.PROFILE}/${user!.username}`}
                    onClick={() => navigate(`${ROUTES.PROFILE}/${user!.username}`)}
                  >
                    <Avatar imageUrl={user!.image} username={user!.username} />
                    {truncateText(user!.username, 12)}
                  </S.HeaderNavItem>
                </li>
              </>
            ) : (
              <>
                <li>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.HOME} onClick={() => navigate(ROUTES.HOME)}>
                    <HomeIcon />
                    Home
                  </S.HeaderNavItem>
                </li>
                <li>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNIN} onClick={() => navigate(ROUTES.SIGNIN)}>
                    Sign in
                  </S.HeaderNavItem>
                </li>
                <li>
                  <S.HeaderNavItem $isActive={pathname === ROUTES.SIGNUP} onClick={() => navigate(ROUTES.SIGNUP)}>
                    Sign up
                  </S.HeaderNavItem>
                </li>
              </>
            )}
          </S.HeaderNavList>
        </S.NavWrapper>
      </S.HeaderContainer>
    </header>
  );
};
