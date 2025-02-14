# Prerequisite

- install python3 
- install pytorch
- install whishper


# How to prepare

## Prepare directories

```
mkdir -p ~/scripts
mkdir -p ~/eval-data/reference
mkdir -p ~/eval-data/student
```

## Build Executor

```
cargo build --release
```

## Copy Script Files

```
cp ./prepare_eval_data.sh ~/scripts/
cp ./run_evaluate.sh ~/scripts/
cp ./target/release/speak-eval ~/scripts/
```

# How to run

You should know two information: w_category, publish_idx.
You can find it from `ceng_publish` table. 


```
~/scripts/run_evaluate.sh <w_category> <publish_idx>
```


## Example

```
~/scripts/run_evaluate.sh S16725293710 620160

```

## Sample query to find information.

```
select *
from ceng_publish as p join ceng_member as m on m.m_id=p.m_id
where p.c_idx='1' and p.w_category='S16725293710' and  p.p_group='S' and p.p_type='HWK'
        and (m.m_id in ('Elsa11', 'andrew11') or m.m_class in ('045002'))
        and p.p_sdate >= '2025-01-28'
order by p.p_sdate desc,p.p_idx desc;
```

```
select * from ceng_speaking_quiz where length(m_id)>1 and publish_idx=620154
```