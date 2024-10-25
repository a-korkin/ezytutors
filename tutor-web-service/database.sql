drop table if exists ezy_course_c4;
create table ezy_course_c4 
(
    course_id serial primary key,
    tutor_id integer not null,
    course_name varchar(140) not null,
    posted_time timestamp default now()
);

insert into ezy_course_c4(course_id, tutor_id, course_name, posted_time)
values
    (1, 1, 'first course', '2020-12-17 05:40:00'),
    (2, 1, 'second course', '2020-12-18 05:45:00');

