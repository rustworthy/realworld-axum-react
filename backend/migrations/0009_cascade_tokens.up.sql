ALTER TABLE "confirmation_tokens" DROP CONSTRAINT token_user_fk;
ALTER TABLE "confirmation_tokens"
  ADD CONSTRAINT token_user_fk FOREIGN KEY (user_id) REFERENCES users ON DELETE CASCADE;
