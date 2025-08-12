import styled from "@emotion/styled";

import { LayoutContainer } from "@/shared/ui/Container";

export const PageWrapper = styled.div`
  width: 100%;
  height: 100%;
  overflow-x: hidden;
`;

export const Banner = styled.div`
  height: 170px;
  color: ${({ theme }) => theme.page.home.bannner.textColor};
  background-color: ${({ theme }) => theme.page.home.bannner.backgroundColor};
`;

export const BannerContainer = styled(LayoutContainer)`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 8px;
`;

export const BannerTitle = styled.h1`
  text-shadow: 0px 1px 3px rgba(0, 0, 0, 0.3);
  font-weight: 700;
  font-size: 56px;
`;

export const BannerDescription = styled.p`
  text-align: center;
  font-weight: 300;
  font-size: 24px;
`;
