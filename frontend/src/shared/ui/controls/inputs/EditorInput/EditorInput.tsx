import { FC } from "react";

import { SrOnly } from "@/shared/ui/SrOnly";
import MDEditor from "@uiw/react-md-editor";
import { useTernaryDarkMode } from "usehooks-ts";

import { FormInputContainer, FormInputError, FormInputErrorContainer } from "../inputs.styles";
import { IEditorInputProps } from "../inputs.types";
import * as S from "./EditorInput.styles";

export const EditorInput: FC<IEditorInputProps> = ({ id, label, placeholder, required, name, value, onChange, error }) => {
  const { isDarkMode } = useTernaryDarkMode();

  return (
    <FormInputContainer>
      <SrOnly>
        <label htmlFor={id}>{label}</label>
      </SrOnly>
      <S.EditorContainer data-color-mode={isDarkMode ? "dark" : "light"} className="container">
        <MDEditor textareaProps={{ placeholder: placeholder ?? label, required, id, name }} value={value} onChange={onChange} />
      </S.EditorContainer>
      <FormInputErrorContainer>
        {error ? <FormInputError id={`${id}_error`}>{error}</FormInputError> : null}
      </FormInputErrorContainer>
    </FormInputContainer>
  );
};
