LOAD CSV WITH HEADERS FROM 'file:///languages.csv' AS row
WITH row WHERE row.id IS NOT NULL
MERGE (c:Language {LanguageId: row.id});