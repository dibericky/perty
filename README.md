# Perty

### Create a new PERT

As a user I can create a new PERT

```
> perty create
> Name: Marketplace
```

### List available PERTs

As a user I can request the list of all available PERTs

```
> perty list
1: Marketplace
2: Social Login
3: Newsfeed
```


### Add activity to an existing 

As a user I can add a new activity with its estimation

```
> perty edit 1 add activity
Add estimated cost:
> Activity: Activity 2
> Optimistic: 18
> Most probable: 25
> Pessimistic: 39
```


### Show detail of a perts

As a user I can get the detail of one of the available PERTs

```
> perty get 1
Name: Marketplace

Report:

Activity | Optimistic | Most probable | Pessimistic | PERT estimate
Activity 1 |   6   |   10  |  15  | 10.166667
Activity 2 |   18   |   25  |  39  | 16.166667
TOTAL  |   |    |    |    96
```


