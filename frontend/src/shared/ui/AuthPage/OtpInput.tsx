import { OTPInput as OTPInputHeadless, SlotProps } from "input-otp";

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

export const OTPInput = () => {
  return (
    <OTPInputHeadless
      maxLength={8}
      containerClassName="group"
      render={({ slots }) => (
        <S.SlotGroupsWrapper>
          <S.SlotGroup>
            {slots.slice(0, 4).map((slot, idx) => (
              <Slot key={idx} {...slot} />
            ))}
          </S.SlotGroup>
          <S.DashLine />
          <S.SlotGroup>
            {slots.slice(4).map((slot, idx) => (
              <Slot key={idx} {...slot} />
            ))}
          </S.SlotGroup>
        </S.SlotGroupsWrapper>
      )}
    />
  );
};
