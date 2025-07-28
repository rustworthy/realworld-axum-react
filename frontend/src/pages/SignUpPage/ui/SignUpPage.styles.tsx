import styled from "styled-components";

export const PageWrapper = styled.div`
  width: 100%;
  height: 100%;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
`;

export const Title = styled.h1`
  margin-bottom: 0.5rem;
  font-family: inherit;
  font-weight: 500;
  font-size: 2.5rem;
  line-height: 1.1;
  color: inherit;
`;

export const SubTitle = styled.a`
  color: #5cb85c;
`;

export const SignUpForm = styled.form`
  width: 100%;
  max-width: 540px;
  padding: 0.5rem 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 16px;
`;

export const FormInput = styled.input`
  display: block;
  width: 100%;
  line-height: 1.25;
  color: #55595c;
  background-color: #fff;
  background-clip: padding-box;
  border: 1px solid rgba(0, 0, 0, 0.15);
  padding: 0.75rem 1.5rem;
  font-size: 1.25rem;
  border-radius: 0.3rem;

  &:focus {
    border-color: #66afe9;
    outline: none;
  }
`;

export const SignUpButtonContainer = styled.div`
  max-width: 120px;
`;
