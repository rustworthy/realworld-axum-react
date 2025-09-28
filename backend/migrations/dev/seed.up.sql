BEGIN;

INSERT INTO users
  (
    user_id,
    username,
    email,
    bio,
    image,
    status,
    password_hash
  )
VALUES
  (
    '00000000-0000-0000-0000-000000000000',
    'rob.pike',
    'rob@golang.org',
    'Robert Pike (born 1956) is a Canadian programmer and author. He is best known for his work on
    the Go programming language while working at Google and the Plan 9 operating system while working at Bell Labs, 
    where he was a member of the Unix team.

    Pike wrote the first window system for Unix in 1981. He is the sole inventor named in a US patent
    for overlapping windows on a computer display.

    With Brian Kernighan, he is the co-author of The Practice of Programming and The Unix Programming Environment.
    With Ken Thompson, he is the co-creator of UTF-8 character encoding',
    'https://upload.wikimedia.org/wikipedia/commons/thumb/9/9c/Rob-pike-oscon.jpg/500px-Rob-pike-oscon.jpg',
    'ACTIVE',
    -- 123123123123
    '$argon2id$v=19$m=19456,t=2,p=1$k23oMd6rxUjGld9wrvr09Q$8qy9ovO+4bTKvgkCreyPiUniOFDIufP4QwUg3euMSSE'
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'steve.klabnik' ,
    'steve@rust.org',
    'You may know me from my work on Rust, or maybe even the stuff I did with Ruby on Rails back in the day.
    You might have read a post I wrote, or maybe even a chapter or two of The Rust Programming Language.
    Maybe you saw a conference talk of mine, or saw one of my (too many,frankly) tweets. I have left Twitter, and now post on BlueSky. (which gets mirrored to the Fediverse via brid.gy as @steveklabnik.com@bsky.brid.gy)
    It’s also possible that you’ve used some code I’ve developed on GitHub.
    Anyway, hi, 👋. I’m Steve.
    If you want to email me, feel free. Please excuse some latency, but I will get back to you eventually.',
    'https://avatars.githubusercontent.com/u/27786?v=4',
    'ACTIVE',
    -- 123123123123
    '$argon2id$v=19$m=19456,t=2,p=1$k23oMd6rxUjGld9wrvr09Q$8qy9ovO+4bTKvgkCreyPiUniOFDIufP4QwUg3euMSSE'
  )
;

INSERT INTO articles
  (
    user_id,
    slug,
    title,
    description,
    body,
    tags
  )
VALUES
  -- Rob's articles
  (
    '00000000-0000-0000-0000-000000000000',
    'lorem-ipsum-dolor-sit-amet',
    'Lorem ipsum dolor sit amet',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'consectetur-adipiscing-elit',
    'Consectetur adipiscing elit',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'uspendisse-eu-iaculis-dui',
    'Uspendisse eu iaculis dui',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'donec-nec-venenatis-mi',
    'Donec nec venenatis mi',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'orci-varius-natoque-penatibus',
    'Orci varius natoque penatibus',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'phasellus-nec-eros-consectetur',
    'Phasellus nec eros consectetur',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'aenean-laoreet-eget-lectus-vitae-fringilla',
    'Aenean laoreet eget lectus vitae fringilla',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000000',
    'sed-molestie-lobortis-metus-non-tincidunt-arcu-posuere-nec',
    'Sed molestie lobortis metus, non tincidunt arcu posuere nec',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  -- Steve's articles
  (
    '00000000-0000-0000-0000-000000000001',
    'lorem-ipsum-dolor-sit-amet-1',
    'Lorem ipsum dolor sit amet 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'consectetur-adipiscing-elit-1',
    'Consectetur adipiscing elit 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'uspendisse-eu-iaculis-dui-1',
    'Uspendisse eu iaculis dui 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'donec-nec-venenatis-mi-1',
    'Donec nec venenatis mi 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'orci-varius-natoque-penatibus-1',
    'Orci varius natoque penatibus 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'phasellus-nec-eros-consectetur-1',
    'Phasellus nec eros consectetur 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'aenean-laoreet-eget-lectus-vitae-fringilla-1',
    'Aenean laoreet eget lectus vitae fringilla 1',
    'description',
    'body',
    ARRAY['one', 'two']
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'sed-molestie-lobortis-metus-non-tincidunt-arcu-posuere-nec-1',
    'Sed molestie lobortis metus, non tincidunt arcu posuere nec 1',
    'description',
    'body',
    ARRAY['one', 'two']
  )
;

COMMIT;
