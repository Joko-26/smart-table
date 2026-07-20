# smart-table

spoken smartable

## get started

1. ensure rust ist installed with `rust -v` if not install it
2. install dependencies `cargo build`
3. copy the contens of `.env-test` to a new `.env` file or use following command: `cp .env-test .env`
4. run with `cargo-watch -x run`

## architecture

### tokens

#### access token

A{MAC-Adress}{creation data + time}{random 10 digit alpha numeric codes}

#### refresh token

R{MAC-Adress}{creation data + time}{random 15 digit alpha numeric codes}

### models

#### User

    id: UUID
    name: str
    email: email(custom)
    grade: int
    admin: bool
    subscription: str
    change: date_time(custom)

#### timetable

    id: UUID
    owner id: UUID
    weeks: array[(day: array[lessons(custom) ], array[times(custom)])]
    change: date_time(custom)

#### subject

    id: UUID
    color: str(hex)
    name: str
    short: str
    main: bool
    change: date_time(custom)

#### lesson 

    id: UUID
    owner id: UUID
    subject: subject: custom
    times: times(custom)
    teacher: str
    room: str
    change: date_time(custom)

#### task 

    id: UUID
    owner id: UUID
    name: str
    description: str
    due date: date(custom)
    create date: date(custom)
    subject: UUID
    priority: int (1-10)
    change: date_time(custom)

#### test

    id: UUID
    owner id: UUID
    name: str
    description: str
    due date: date(custom)
    create date: date(custom)
    subject: UUID
    notifications: (hmm)
    change: date_time(custom)

#### config 

    user id: UUID
    mode: str
    theme: str
    icon: str
    notifications: (hmm)
    change: date_time(custom)

### routes

- /tasks/{user-id} (GET, PUT, POST, DELETE)
- /tests/{user-id} (GET, PUT, POST, DELETE)
- /user/{id} (GET, PUT, POST)
- /timetable/{user-id} (GET, PUT, POST, DELETE)
- /sujects/{user-id} (GET, PUT, POST, DELETE)
- /mix/tasks/{user-id} (GET)
- /mix/tests/{user-id} (GET)
- /mix/user/{id} (GET)
- /mix/timetable/{user-id} (GET)
