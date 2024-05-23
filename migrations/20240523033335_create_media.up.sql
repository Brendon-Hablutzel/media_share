create table if not exists media (
    label varchar(50) primary key,
    file_location varchar(128) not null,
    content_type varchar(20) not null,
    expiry timestamptz not null
);
