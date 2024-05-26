create table if not exists media (
    label text primary key,
    content_type text not null,
    expiry timestamptz not null
);
