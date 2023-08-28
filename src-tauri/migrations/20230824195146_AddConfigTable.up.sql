-- Add up migration script here
create table config (
    id integer primary key,
    prompt_disable_package boolean NOT NULL,
    custom_adb_path TEXT NOT NULL,
    clear_packages_on_disable boolean NOT NULL
);