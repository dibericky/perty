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

```

