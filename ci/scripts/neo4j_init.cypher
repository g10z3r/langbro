// Загрузка данных из файла
LOAD CSV WITH HEADERS FROM 'https://gist.githubusercontent.com/I0HuKc/69ae9817a376e677c8ff0e963ab8b252/raw/67fefca3ec3a775ae8cdcdebe6d76b5067795414/languages.csv' AS row
WITH row WHERE row.name IS NOT NULL
MERGE (c:Language {code: row.name});

