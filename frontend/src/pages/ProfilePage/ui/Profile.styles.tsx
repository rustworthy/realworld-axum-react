import styled from "@emotion/styled";

import { LayoutContainer } from "@/shared/ui/Container";

export const PageWrapper = styled.div`
  width: 100%;
  height: 100%;
  background-color: ${({ theme }) => theme.page.article.content.backgroundColor};
  color: ${({ theme }) => theme.page.article.content.textColor};
`;

export const Banner = styled.div`
  background: ${({ theme }) => theme.page.article.banner.backgroundColor};
  color: ${({ theme }) => theme.page.article.banner.textColor};
  padding: 2rem 0.5rem;
`;

export const BannerContainer = styled(LayoutContainer)`
  max-width: 1000px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
`;

export const MainContent = styled(LayoutContainer)`
  max-width: 1000px;
  padding: 2rem 0.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

export const FeedContainer = styled.div`
  width: 100%;
  max-width: 50rem;
  display: flex;
  flex-direction: column;
`;

export const ProfileMeta = styled.div`
  display: flex;
  align-items: center;
  gap: 2rem;
  flex-wrap: wrap;
`;

export const ProfileTitle = styled.h1`
  font-weight: 600;
  font-size: 1.5rem;
  margin: 0;
  color: inherit;
`;

export const ProfileBio = styled.p`
  text-align: center;
  max-width: 40rem;
  padding: 0 1rem;
`;

export const ProfileActions = styled.div`
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
`;

export const PreviewList = styled.div`
  display: flex;
  flex-direction: column;
`;
