import { FC } from "react";

import { OTPInput as OTPInputHeadless, SlotProps } from "input-otp";

import { FormInputContainer, FormInputError } from "../inputs.styles";
import { IOTPInputProps } from "../inputs.types";
import * as S from "./OtpInput.styles";

const FakeCaret = () => {
  return (
    <S.FakeCaretContainer>
      <S.CaretLine />
    </S.FakeCaretContainer>
  );
};

const Slot = (props: SlotProps) => {
  return (
    <S.Slot isActive={props.isActive}>
      <S.CharContainer>{props.char ?? props.placeholderChar}</S.CharContainer>
      {props.hasFakeCaret && <FakeCaret />}
    </S.Slot>
  );
};

export const OTPInput: FC<IOTPInputProps> = ({ id, length, error, label, ...field }) => {
  const splitGroupsAt = Math.floor(length / 2);
  return (
    <FormInputContainer>
      <OTPInputHeadless
        {...field}
        id={id}
        aria-label={label}
        maxLength={length}
        containerClassName="group"
        aria-invalid={!!error}
        aria-errormessage={`${id}_error`}
        render={({ slots }) => (
          <S.SlotGroupsWrapper>
            <S.SlotGroup>
              {slots.slice(0, splitGroupsAt).map((slot, idx) => (
                <Slot key={idx} {...slot} />
              ))}
            </S.SlotGroup>
            <S.DashLine />
            <S.SlotGroup>
              {slots.slice(splitGroupsAt).map((slot, idx) => (
                <Slot key={idx} {...slot} />
              ))}
            </S.SlotGroup>
          </S.SlotGroupsWrapper>
        )}
      />
      {error ? <FormInputError id={`${id}_error`}>{error}</FormInputError> : null}
    </FormInputContainer>
  );
};
