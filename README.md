# Perty

### Create a new PERT

As a user I can create a new PERT

```
> cargo run create
> Name: Marketplace
```

### List available PERTs

As a user I can request the list of all available PERTs

```
> cargo run list
1: Marketplace
2: Social Login
3: Newsfeed
```


### Add activity to an existing 

As a user I can add a new activity with its estimation

```
> cargo run edit 1 add activity
Add estimated cost:
> Activity: Activity 2
> Optimistic: 18
> Most probable: 25
> Pessimistic: 39
```


### Show detail of a perts

As a user I can get the detail of one of the available PERTs

```
> cargo run get 1
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
> cargo run get 1 --html
```

A `report-PERT_ID.html` file will be created and automatically opened in the browser.

Or generate as CSV file

```
> cargo run get 1 --csv
```

### Set dependencies between activities

```
> edit PERT_ID add dependency
1 activity name1
2 activity name2
3 activity name 3
"A" depends on "B"
Select activity "A":   
> 2
Select activity which depends on "activity name2": 
> activity name3
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
	CONSTRAINT activity_dependencies_fk FOREIGN KEY (activity_id_head) REFERENCES public.activities(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT activity_dependencies_fk_1 FOREIGN KEY (activity_id_tail) REFERENCES public.activities(id) ON DELETE CASCADE ON UPDATE CASCADE
);


```

