drop table if exists ezy_course_c5;
create table ezy_course_c5
(
    course_id serial primary key,
    tutor_id integer not null,
    course_name varchar(140) not null,
    posted_time timestamp default now()
);

insert into ezy_course_c5(course_id, tutor_id, course_name, posted_time)
values
    (1, 1, 'first course', '2021-03-17 05:40:00'),
    (2, 1, 'second course', '2021-03-18 05:45:00');
