import { FC, PropsWithChildren } from "react";

import * as S from "./FormPage.styles";

export const Container: FC<PropsWithChildren<{ title: string }>> = (props) => {
  return (
    <S.PageWrapper>
      <S.Title>{props.title}</S.Title>
      {props.children}
    </S.PageWrapper>
  );
};

export const Form = S.Form;

export const FormSubmissionSection = S.FormSubmissionSection;
