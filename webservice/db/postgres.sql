CREATE TABLE public.course (
	id serial4 NOT NULL,
	teacher_id int4 NULL,
	name varchar(140) NULL,
	"time" timestamp DEFAULT now() NULL,
	CONSTRAINT course_pk PRIMARY KEY (id)
);