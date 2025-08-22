import styled from "@emotion/styled";

export const EditorContainer = styled.div`
  .w-md-editor {
    border: 1px solid transparent;

    &:focus-within {
      border: 1px solid ${({ theme }) => theme.shared.input.borderColorFocused};
    }
  }

  .w-md-editor-content {
    padding-inline: 1rem;
  }

  .wmde-markdown,
  .wmde-markdown-var {
    --md-editor-font-family: "Source Sans Pro", sans-serif;
    --color-border-default: ${({ theme }) => theme.shared.input.borderColor};
    --color-canvas-default: ${({ theme }) => theme.shared.input.backgroundColor};
  }
`;
