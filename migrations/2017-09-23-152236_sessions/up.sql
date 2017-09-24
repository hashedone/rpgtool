create table sessions (
    id text primary key not null,
    user integer not null,
    expires datetime not null,
    foreign key(user) references users(id)
)
