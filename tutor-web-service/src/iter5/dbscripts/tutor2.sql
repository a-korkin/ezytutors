alter table ezy_course_c6 
add constraint fk_tutor foreign key(tutor_id) 
references ezy_tutor_c6(tutor_id) on delete cascade;

insert into ezy_tutor_c6(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
values
    (1, 'merlenel', 'http://s3.amazon.aws.com/pic1', 'merlene is an experienced professional'),
    (2, 'frank', 'http://s3.amazon.aws.com/pic2', 'frank is an expert nuclear engineer');

insert into ezy_course_c6(course_id, tutor_id, course_name, course_level, posted_time)
values
    (1, 1, 'first course', 'beginner', '2021-04-12 05:40:00'),
    (2, 1, 'second course', 'ebook', '2021-04-12 05:45:00');
