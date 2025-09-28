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
    'rob.spike',
    'rob@golang.org',
    'Robert Spike (born 1956) is a Canadian programmer and author. He is best known for his work on
the Go programming language while working at Google and the Plan 9 operating system while working at Bell Labs, 
where he was a member of the Unix team.

Spike wrote the first window system for Unix in 1981. He is the sole inventor named in a US patent
for overlapping windows on a computer display.

With Brian Kernighan, he is the co-author of The Practice of Programming and The Unix Programming Environment.
With Ken Thompson, he is the co-creator of UTF-8 character encoding',
    'https://upload.wikimedia.org/wikipedia/commons/thumb/6/6c/Sam_Worthington_2013.jpg/330px-Sam_Worthington_2013.jpg',
    'ACTIVE',
    -- 123123123123
    '$argon2id$v=19$m=19456,t=2,p=1$k23oMd6rxUjGld9wrvr09Q$8qy9ovO+4bTKvgkCreyPiUniOFDIufP4QwUg3euMSSE'
  ),
  (
    '00000000-0000-0000-0000-000000000001',
    'awesome.steve' ,
    'steve@rust.org',
    'You may know me from my work on Rust, or maybe even the stuff I did with Ruby on Rails back in the day.
You might have read a post I wrote, or maybe even a chapter or two of The Rust Programming Language.
Maybe you saw a conference talk of mine, or saw one of my (too many,frankly) tweets. I have left Twitter, and now post on BlueSky. (which gets mirrored to the Fediverse via brid.gy as @steveklabnik.com@bsky.brid.gy)
It’s also possible that you’ve used some code I’ve developed on GitHub.
Anyway, hi, 👋. I’m Steve.

If you want to email me, feel free. Please excuse some latency, but I will get back to you eventually.',
    'https://upload.wikimedia.org/wikipedia/commons/thumb/6/6b/Charles_Babbage_-_1860.jpg/500px-Charles_Babbage_-_1860.jpg',
    'ACTIVE',
    -- 123123123123
    '$argon2id$v=19$m=19456,t=2,p=1$k23oMd6rxUjGld9wrvr09Q$8qy9ovO+4bTKvgkCreyPiUniOFDIufP4QwUg3euMSSE'
  ),
 (
    '00000000-0000-0000-0000-000000000002',
    'Kate1337' ,
    'kate@tokio.org',
    'Hi I am Kate, I a technical writer.',
    'https://upload.wikimedia.org/wikipedia/commons/thumb/4/4c/Ada_Lovelace_daguerreotype_by_Antoine_Claudet_1843_-_cropped.png/500px-Ada_Lovelace_daguerreotype_by_Antoine_Claudet_1843_-_cropped.png',
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
  -- ############################## ARTICLE 1 #################################
  (
    '00000000-0000-0000-0000-000000000000',
    'the-evolution-of-programming-languages',
    'The Evolution of Programming Languages',
    'A deep dive into how programming languages have evolved from assembly to modern high-level languages,
    exploring the design decisions that shaped Go and other influential languages.',
    '# The Evolution of Programming Languages

Programming languages have transformed from primitive assembly to sophisticated high-level languages,
each generation building on lessons from predecessors.

![Computer History](https://upload.wikimedia.org/wikipedia/commons/thumb/0/0c/ENIAC-changing_a_tube_%28cropped%29.jpg/330px-ENIAC-changing_a_tube_%28cropped%29.jpg)

## Key Milestones

[FORTRAN](https://en.wikipedia.org/wiki/Fortran) (1950s) revolutionized scientific computing,
while [COBOL](https://en.wikipedia.org/wiki/COBOL) served business applications.
The [C programming language](https://en.wikipedia.org/wiki/C_(programming_language)) at Bell Labs
established the "simple tools" philosophy that became foundational to software engineering.

![Bell Labs](https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Bell_telephone_laboratories_logo.svg/320px-Bell_telephone_laboratories_logo.svg.png)

## Modern Design

Today''s languages like [Go](https://en.wikipedia.org/wiki/Go_(programming_language)) prioritize readability,
maintainability, and efficient concurrency. The future will focus on making concurrent programming more accessible
while maintaining simplicity.',
    ARRAY['programming', 'history', 'go', 'design']

  ),
  -- ############################## ARTICLE 2 #################################
  (
    '00000000-0000-0000-0000-000000000001',
    'building-distributed-systems-at-scale',
    'Building Distributed Systems at Scale',
    'Lessons learned from building large-scale distributed systems at Google, including design patterns, trade-offs, and common pitfalls to avoid.',
    '# Building Distributed Systems at Scale

Scaling systems to millions of users requires fundamental architectural shifts.
At [Google](https://en.wikipedia.org/wiki/Google), they have learned key lessons
from building global infrastructure.

![Google Data Center](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2d/Google_datacenter_%282007%29_-_panoramio_-_erwinboogert_%282%29.jpg/500px-Google_datacenter_%282007%29_-_panoramio_-_erwinboogert_%282%29.jpg)

## Core Principles

The [CAP theorem](https://en.wikipedia.org/wiki/CAP_theorem) forces trade-offs between Consistency, Availability, and Partition tolerance. Essential principles:

1. **Embrace Failure**: Use circuit breakers, timeouts, graceful degradation
2. **Monitor Everything**: Comprehensive observability is crucial
3. **Keep It Simple**: Favor well-understood solutions over clever ones

The [MapReduce](https://en.wikipedia.org/wiki/MapReduce) model demonstrates these principles by providing simple abstractions for distributed processing while handling failures transparently.',
    ARRAY['distributed-systems', 'google', 'scalability', 'architecture']
  ),

  -- ############################## ARTICLE 3 #################################
  (
    '00000000-0000-0000-0000-000000000002', -- user_id
    'the-philosophy-of-unix-design',
    'The Philosophy of Unix Design',
    'Exploring the timeless design principles that made Unix successful and how they continue to influence modern software development.',
    '# The Philosophy of Unix Design

[Unix](https://en.wikipedia.org/wiki/Unix), developed at Bell Labs in the 1970s, introduced timeless design principles that continue influencing modern software development.

![Unix Timeline](https://upload.wikimedia.org/wikipedia/commons/thumb/7/77/Unix_history-simple.svg/640px-Unix_history-simple.svg.png)

## Core Principles

1. **Do One Thing Well**: Single-purpose programs are easier to understand and combine
2. **Work Together**: Standard interfaces enable unexpected combinations
3. **Handle Text Streams**: Universal interface for human and machine processing

## Modern Impact

These principles directly influenced [Go](https://en.wikipedia.org/wiki/Go_(programming_language)): simplicity over cleverness, composition through interfaces, and clear conventions.

![Ken Thompson](https://upload.wikimedia.org/wikipedia/commons/d/dd/Ken_Thompson%2C_2019.jpg)

Good software design is timeless—tools built on Unix principles remain useful decades later.',
    ARRAY['unix', 'philosophy', 'design', 'go', 'programming']
  ),

  -- ############################## ARTICLE 4 #################################
  (
    '00000000-0000-0000-0000-000000000002', -- user_id
    'lessons-from-plan9-operating-system',
    'Lessons from Plan 9 Operating System',
    'How the experimental Plan 9 operating system pushed the boundaries of distributed computing and influenced modern cloud architectures.',
    '# Lessons from Plan 9 Operating System

[Plan 9](https://en.wikipedia.org/wiki/Plan_9_from_Bell_Labs) was Bell Labs'' experimental Unix successor. Though never mainstream, its distributed computing ideas were decades ahead of their time.

## Key Innovations

Plan 9 extended "everything is a file" to network resources, enabling true network transparency. Programs accessed remote resources as easily as local ones.

**Core features:**
- **9P Protocol**: Simple, efficient remote resource access
- **Union Directories**: Multiple filesystems at one location
- **Per-Process Namespaces**: Individual filesystem views

## Modern Legacy

Plan 9 concepts appear in contemporary systems like [Kubernetes](https://en.wikipedia.org/wiki/Kubernetes), microservices, and service meshes. [UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoding was also developed for Plan 9.

Plan 9 proved operating systems could be fundamentally different, paving the way for today''s [cloud computing](https://en.wikipedia.org/wiki/Cloud_computing) architectures.',
    ARRAY['plan9', 'distributed-computing', 'bell-labs', 'innovation', 'os']
  ),


  -- ############################## ARTICLE 5 #################################
  (
    '00000000-0000-0000-0000-000000000000',
    'concurrency-patterns-in-go',
    'Concurrency Patterns in Go',
    'A comprehensive guide to Go''s concurrency primitives and patterns, from basic goroutines to advanced synchronization techniques.',
    '# Concurrency Patterns in Go

[Go](https://en.wikipedia.org/wiki/Go_(programming_language)) treats concurrency as a first-class citizen, making concurrent programming easier than traditional thread-based models.

![Gopher Mascot](https://upload.wikimedia.org/wikipedia/commons/thumb/0/05/Go_Logo_Blue.svg/320px-Go_Logo_Blue.svg.png)

## Core Concepts

**Goroutines** are lightweight threads—thousands can run efficiently on few OS threads:
```go
go func() {
    fmt.Println("Hello from goroutine!")
}()
```

**Channels** enable safe communication: "Don''t communicate by sharing memory; share memory by communicating."

## Common Patterns
- **Pipeline**: Chain goroutines with channels
- **Fan-out/Fan-in**: Distribute and collect work
- **Worker Pool**: Fixed goroutines processing tasks

These patterns excel in [web servers](https://en.wikipedia.org/wiki/Web_server), data pipelines, and distributed systems. Go''s design is inspired by Tony Hoare''s 1978 [CSP model](https://en.wikipedia.org/wiki/Communicating_sequential_processes)—proving good ideas are timeless.',
    ARRAY['go', 'concurrency', 'goroutines', 'channels', 'programming']
  ),


  -- ############################## ARTICLE 6 #################################
  (
    '00000000-0000-0000-0000-000000000000', -- user_id
    'the-art-of-readable-code',
    'The Art of Readable Code',
    'Why code readability matters more than clever optimizations, and practical techniques for writing code that your future self will thank you for.',
    '# The Art of Readable Code

Code is read far more than it''s written—a program may be written once but read hundreds of times. This truth should guide every programming decision.

## Why It Matters

Readable code is easier to debug, faster to modify, and less error-prone. Clear intent reduces bugs.

## Key Principles

1. **Choose names carefully**: `calculateTotalPrice()` vs `calc()`
2. **Self-documenting code**: Comments should explain *why*, not *what*
3. **Small functions**: One responsibility per function

## Language Design

[Go](https://en.wikipedia.org/wiki/Go_(programming_language)) prioritizes readability with consistent formatting (`gofmt`), minimal syntax, and explicit error handling. [Python](https://en.wikipedia.org/wiki/Python_(programming_language)) shares this philosophy.

Remember: You''re writing for the next programmer who maintains your code—and that might be you.',
    ARRAY['readability', 'clean-code', 'best-practices', 'programming']
  ),

  -- ############################## ARTICLE 7 #################################
  (
    '00000000-0000-0000-0000-000000000001', -- user_id
    'debugging-complex-systems',
    'Debugging Complex Systems',
    'Effective strategies for finding and fixing bugs in large, distributed systems where traditional debugging approaches fall short.',
    '# Debugging Complex Systems

Debugging distributed systems requires different approaches than single-threaded programs.

When systems span multiple services and networks, traditional debugging becomes impractical.

## Scientific Debugging Method

1. **Observe**: Gather data from logs, metrics, traces
2. **Hypothesize**: Form testable theories
3. **Experiment**: Design controlled tests
4. **Conclude**: Fix the bug or refine hypothesis

## Essential Tools

- **Structured Logging**: Consistent formats with correlation IDs
- **Distributed Tracing**: [OpenTelemetry](https://en.wikipedia.org/wiki/OpenTelemetry) for request visualization
- **Circuit Breakers**: Prevent cascade failures

![Google SRE](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2f/Google_2015_logo.svg/320px-Google_2015_logo.svg.png)

[Production outages](https://en.wikipedia.org/wiki/Outage) teach the importance of monitoring, graceful degradation, rollback procedures, and blameless post-mortems.

Key insight: Build observable systems by design. You can''t debug what you can''t see.',
    ARRAY['debugging', 'programming', 'distributed-systems', 'observability', 'sre']
  )
;

COMMIT;
