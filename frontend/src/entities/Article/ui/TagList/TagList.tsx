import { FC } from "react";

import * as S from "./TagsList.styles";

export interface ITagsListProps {
  tagClassName?: string;
  tags: string[];
  onClick?: (tag: ITagsListProps["tags"][number]) => void;
}

export const TagList: FC<ITagsListProps> = ({ tagClassName, tags, onClick }) => {
  return (
    <S.TagList>
      {tags.map((tag) => (
        <S.Tag $interactive={!!onClick} onClick={() => onClick?.(tag)} key={tag} className={tagClassName}>
          {tag}
        </S.Tag>
      ))}
    </S.TagList>
  );
};
