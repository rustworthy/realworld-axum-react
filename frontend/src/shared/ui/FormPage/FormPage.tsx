import { FC, PropsWithChildren } from "react";

import * as S from "./FormPage.styles";

const Container: FC<PropsWithChildren<{ title: string }>> = (props) => {
  return (
    <S.PageWrapper>
      <S.Title>{props.title}</S.Title>
      {props.children}
    </S.PageWrapper>
  );
};

export const FormPage = {
  Container,
  Form: S.Form,
  FormSubmissionSection: S.FormSubmissionSection,
}
