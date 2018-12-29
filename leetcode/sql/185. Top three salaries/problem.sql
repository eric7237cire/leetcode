
select Department, Employee, Salary
FROM
(
Select d.name as Department, e.name as Employee, e.Salary,
Rank()
          over (Partition BY d.id
                ORDER BY e.salary DESC ) as rnk
from employee e inner join department d on e.departmentid = d.id
) sq where sq.rnk <= 3