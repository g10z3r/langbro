LOAD CSV WITH HEADERS FROM 'file:///data.csv' AS row
WITH row WHERE row.Id IS NOT NULL
MERGE (c:Company {companyId: row.Id});