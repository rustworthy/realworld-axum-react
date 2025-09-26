import { FC } from "react";

import * as S from "./TagsList.styles";

export type TagsListProps = {
  tagClassName?: string;
  tags: string[];
  onClick?: (tag: TagsListProps["tags"][number]) => void;
};

export const TagList: FC<TagsListProps> = ({ tagClassName, tags, onClick }) => {
  return (
    <S.TagList>
      {tags.map((tag) => (
        <S.Tag $interactive={!!onClick} key={tag} className={tagClassName}>
          {tag}
        </S.Tag>
      ))}
    </S.TagList>
  );
};
