# PERTy

### Create a new PERT

As a user I can create a new PERT

```
> create
> Name: Marketplace
```

### List available PERTs

As a user I can request the list of all available PERTs

```
> list
1: Marketplace
2: Social Login
3: Newsfeed
```


### Add activity to an existing 

As a user I can add a new activity with its estimation

```
> edit 1 add activity
Add estimated cost:
> Activity: Activity 2
> Optimistic: 18
> Most probable: 25
> Pessimistic: 39
```


### Show detail of a perts

As a user I can get the detail of one of the available PERTs

```
> get 1 pert
Name: Marketplace

+------------+------------+----------+-------------+-----------------+
| Activity   | Optimistic | Probable | Pessimistic | PERT estimation |
+------------+------------+----------+-------------+-----------------+
| activity 1 | 10         | 20       | 30          | 20              |
+------------+------------+----------+-------------+-----------------+
| activity 2 | 15         | 20       | 40          | 22.5            |
+------------+------------+----------+-------------+-----------------+

TOTAL: 42.5
```

Or generate as HTML file

```
> get 1 pert --html
```

A `report-PERT_ID.html` file will be created and automatically opened in the browser.

Or generate as CSV file

```
> get 1 pert --csv
```

### Set dependencies between activities

```
> edit PERT_ID add dependency
1 activity Foobar
2 activity Lorem
3 activity Ipsum
4 activity Dolorem
"A" depends on "B"
Insert the ID of activity "A":   
> 2
Insert the ID of activity "B": 
> 3
```

### Get Roadmap of PERT

Roadmap of a PERT is based on dependency between activities. 
All activities put in a phase are indipendent between each other but each of them depends on an activity of the phase before.

```
> get PERT_ID pert

Hello, welcome to Perty!
Calculating roadmap for PERT 1
Phase #1
+----+---------------------+
| ID | Name                |
+----+---------------------+
|  1 | activity Foobar     |
+----+---------------------+
|  3 | activity Ipsum      |
+----+---------------------+


Phase #2
+----+-------------------------+
| ID | Name                    |
+----+-------------------------+
|  2 | activity Lorem          |
+----+-------------------------+


Phase #3
+----+------------------+
| ID | Name             |
+----+------------------+
|  4 | activity Dolorem |
+----+------------------+
```

## Github Integration

In order to use these features you need to set `GITHUB_ACCESS_TOKEN` environment variable. Go [here](https://github.com/settings/tokens) to get your access token.

### Create Project Board for PERT

Read [here](https://docs.github.com/en/issues/organizing-your-work-with-project-boards/managing-project-boards/about-project-boards) for more information.


```
>create board --github
1: Marketplace
2: Social Login
3: Newsfeed
Select a PERT
> 1
Github repository url: (e.g.: https://github.com/dibericky/perty)
> https://github.com/foobar/lorem
Creating board...
Board created: https://github.com/api-playground/projects-test/projects/1
```

### Create PERT Task on Project Board (Coming Soon)

```
>create task --github
List of projects with Github Board. Select one:
1: Marketplace
3: Newsfeed
> 1
Your task are creating... Please, wait...

Creating task for: activity Foobar
Task created: https://github.com/api-playground/projects-test/projects/2#card-83518444

Creating task for: activity Lorem
Task created: https://github.com/api-playground/projects-test/projects/2#card-83518445

All task have been created.
```



## PostgreSQL

```
CREATE TABLE public.pert (
	"name" varchar NOT NULL,
	id serial4 NOT NULL,
	CONSTRAINT pert_pk PRIMARY KEY (id)
);

CREATE TABLE public.activities (
	pert_id serial4 NOT NULL,
	pessimistic int4 NOT NULL,
	probable int4 NOT NULL,
	optimistic int4 NOT NULL,
	id serial4 NOT NULL,
	"name" varchar NOT NULL,
	CONSTRAINT activities_pk PRIMARY KEY (id),
	CONSTRAINT activities_fk FOREIGN KEY (pert_id) REFERENCES public.pert(id) ON DELETE CASCADE
);

CREATE TABLE public.activity_dependencies (
	activity_id_head int4 NOT NULL DEFAULT nextval('activity_dependencies_activity_id_a_seq'::regclass),
	activity_id_tail int4 NOT NULL DEFAULT nextval('activity_dependencies_activity_id_b_seq'::regclass),
	CONSTRAINT activity_dependencies_pk PRIMARY KEY (activity_id_head, activity_id_tail),
	CONSTRAINT activity_dependencies_fk FOREIGN KEY (activity_id_head) REFERENCES public.activities(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT activity_dependencies_fk_1 FOREIGN KEY (activity_id_tail) REFERENCES public.activities(id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE public.boards (
	pert_id serial4 NOT NULL,
	github_board_id serial4 NOT NULL,
	CONSTRAINT boards_pk PRIMARY KEY (github_board_id, pert_id),
	CONSTRAINT boards_fk FOREIGN KEY (pert_id) REFERENCES public.pert(id) ON DELETE CASCADE ON UPDATE CASCADE
);

```

