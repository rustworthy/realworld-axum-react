import { FC } from "react";

import * as S from "./TagsList.styles";

export type TagsListProps = {
  tags: string[];
  onClick?: (tag: TagsListProps["tags"][number]) => void;
};

export const TagList: FC<TagsListProps> = ({ tags, onClick }) => {
  return (
    <S.TagList>
      {tags.map((tag) => (
        <S.Tag $interactive={!!onClick} key={tag}>
          {tag}
        </S.Tag>
      ))}
    </S.TagList>
  );
};
